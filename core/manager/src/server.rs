//! Multiplayer server list — fetch and manage OpenTTD multiplayer servers.
//!
//! Uses the OpenTTD master server API to discover active game servers,
//! their status, player counts, and connection details.

use serde::{Deserialize, Serialize};
use crate::Result;

/// Connection status of a server
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ServerStatus {
    Online,
    Offline,
    Full,
    PasswordRequired,
}

/// A multiplayer server entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerEntry {
    /// Server IP address or hostname
    pub host: String,
    /// Server port (default: 3979)
    pub port: u16,
    /// Server name
    pub name: String,
    /// Current version running on the server
    pub version: String,
    /// Current number of players
    pub players_current: u16,
    /// Maximum number of players
    pub players_max: u16,
    /// Number of spectators
    pub spectators: u16,
    /// Whether the server requires a password
    pub passworded: bool,
    /// Current map name
    pub map_name: String,
    /// Map size in tiles
    pub map_size: (u16, u16),
    /// Current game date
    pub game_date: Option<String>,
    /// Server status
    pub status: ServerStatus,
    /// Latency in milliseconds (measured locally)
    pub latency_ms: Option<u64>,
    /// Server description/motd
    pub description: Option<String>,
    /// Country code (ISO 3166-1 alpha-2)
    pub country: Option<String>,
}

/// Filter for server list queries
#[derive(Debug, Clone, Default)]
pub struct ServerFilter {
    pub version: Option<String>,
    pub min_players: Option<u16>,
    pub max_players: Option<u16>,
    pub passworded: Option<bool>,
    pub name_contains: Option<String>,
    pub country: Option<String>,
}

/// Master server URLs for OpenTTD
const MASTER_SERVER_URL: &str = "https://master.openttd.org/";
const SERVER_LIST_URL: &str = "https://master.openttd.org/servers";

/// Server list fetcher
pub struct ServerListFetcher {
    client: reqwest::Client,
}

impl ServerListFetcher {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    /// Fetch the list of public multiplayer servers
    pub async fn fetch_servers(&self) -> Result<Vec<ServerEntry>> {
        let response = self.client
            .get(SERVER_LIST_URL)
            .timeout(std::time::Duration::from_secs(10))
            .send()
            .await?;

        let text = response.text().await?;
        let servers = self.parse_server_list(&text);
        Ok(servers)
    }

    /// Fetch servers with a filter applied
    pub async fn fetch_filtered(&self, filter: &ServerFilter) -> Result<Vec<ServerEntry>> {
        let mut servers = self.fetch_servers().await?;
        servers.retain(|s| filter.matches(s));
        Ok(servers)
    }

    /// Parse the server list from master server response
    fn parse_server_list(&self, data: &str) -> Vec<ServerEntry> {
        let mut servers = Vec::new();

        for line in data.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            // Format: host:port:name:version:players:max:spectators:passworded
            let parts: Vec<&str> = line.splitn(8, ':').collect();
            if parts.len() >= 7 {
                let host = parts[0].to_string();
                let port = parts[1].parse::<u16>().unwrap_or(3979);
                let name = parts[2].to_string();
                let version = parts[3].to_string();
                let players = parts[4].parse::<u16>().unwrap_or(0);
                let max = parts[5].parse::<u16>().unwrap_or(8);
                let spectators = parts[6].parse::<u16>().unwrap_or(0);
                let passworded = parts.get(7).map_or(false, |&p| p == "1");

                let status = if passworded {
                    ServerStatus::PasswordRequired
                } else if players >= max {
                    ServerStatus::Full
                } else {
                    ServerStatus::Online
                };

                servers.push(ServerEntry {
                    host,
                    port,
                    name,
                    version,
                    players_current: players,
                    players_max: max,
                    spectators,
                    passworded,
                    map_name: String::new(),
                    map_size: (0, 0),
                    game_date: None,
                    status,
                    latency_ms: None,
                    description: None,
                    country: None,
                });
            }
        }

        servers
    }
}

impl ServerFilter {
    /// Check if a server matches this filter
    fn matches(&self, server: &ServerEntry) -> bool {
        if let Some(ref ver) = self.version {
            if !server.version.contains(ver) {
                return false;
            }
        }
        if let Some(min) = self.min_players {
            if server.players_current < min {
                return false;
            }
        }
        if let Some(max) = self.max_players {
            if server.players_current > max {
                return false;
            }
        }
        if let Some(pw) = self.passworded {
            if server.passworded != pw {
                return false;
            }
        }
        if let Some(ref name) = self.name_contains {
            if !server.name.to_lowercase().contains(&name.to_lowercase()) {
                return false;
            }
        }
        if let Some(ref country) = self.country {
            if server.country.as_deref() != Some(country) {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_server_list() {
        let data = "127.0.0.1:3979:Test Server:14.1:3:8:1:0\n\
                    192.168.1.1:3979:Another Server:14.0:0:8:0:1";
        let fetcher = ServerListFetcher::new();
        let servers = fetcher.parse_server_list(data);
        assert_eq!(servers.len(), 2);
        assert_eq!(servers[0].name, "Test Server");
        assert_eq!(servers[0].players_current, 3);
        assert_eq!(servers[0].players_max, 8);
        assert_eq!(servers[0].status, ServerStatus::Online);
        assert_eq!(servers[1].status, ServerStatus::PasswordRequired);
    }

    #[test]
    fn test_parse_ignores_comments() {
        let data = "# This is a comment\n127.0.0.1:3979:My Server:14.1:1:8:0:0\n";
        let fetcher = ServerListFetcher::new();
        let servers = fetcher.parse_server_list(data);
        assert_eq!(servers.len(), 1);
    }

    #[test]
    fn test_filter_by_version() {
        let servers = vec![
            ServerEntry { host: "a".into(), port: 3979, name: "Server A".into(), version: "14.1".into(), players_current: 2, players_max: 8, spectators: 0, passworded: false, map_name: String::new(), map_size: (0,0), game_date: None, status: ServerStatus::Online, latency_ms: None, description: None, country: None },
            ServerEntry { host: "b".into(), port: 3979, name: "Server B".into(), version: "13.4".into(), players_current: 5, players_max: 8, spectators: 0, passworded: false, map_name: String::new(), map_size: (0,0), game_date: None, status: ServerStatus::Online, latency_ms: None, description: None, country: None },
        ];
        let filter = ServerFilter { version: Some("14".into()), ..Default::default() };
        let filtered: Vec<_> = servers.into_iter().filter(|s| filter.matches(s)).collect();
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].version, "14.1");
    }

    #[test]
    fn test_filter_by_name() {
        let servers = vec![
            ServerEntry { host: "a".into(), port: 3979, name: "My Cool Server".into(), version: "14.1".into(), players_current: 2, players_max: 8, spectators: 0, passworded: false, map_name: String::new(), map_size: (0,0), game_date: None, status: ServerStatus::Online, latency_ms: None, description: None, country: None },
            ServerEntry { host: "b".into(), port: 3979, name: "Another Server".into(), version: "14.0".into(), players_current: 5, players_max: 8, spectators: 0, passworded: false, map_name: String::new(), map_size: (0,0), game_date: None, status: ServerStatus::Online, latency_ms: None, description: None, country: None },
        ];
        let filter = ServerFilter { name_contains: Some("Cool".into()), ..Default::default() };
        let filtered: Vec<_> = servers.into_iter().filter(|s| filter.matches(s)).collect();
        assert_eq!(filtered.len(), 1);
    }

    #[test]
    fn test_filter_passworded() {
        let servers = vec![
            ServerEntry { host: "a".into(), port: 3979, name: "Server A".into(), version: "14.1".into(), players_current: 2, players_max: 8, spectators: 0, passworded: true, map_name: String::new(), map_size: (0,0), game_date: None, status: ServerStatus::PasswordRequired, latency_ms: None, description: None, country: None },
            ServerEntry { host: "b".into(), port: 3979, name: "Server B".into(), version: "14.0".into(), players_current: 5, players_max: 8, spectators: 0, passworded: false, map_name: String::new(), map_size: (0,0), game_date: None, status: ServerStatus::Online, latency_ms: None, description: None, country: None },
        ];
        let filter = ServerFilter { passworded: Some(false), ..Default::default() };
        let filtered: Vec<_> = servers.into_iter().filter(|s| filter.matches(s)).collect();
        assert_eq!(filtered.len(), 1);
        assert!(!filtered[0].passworded);
    }
}