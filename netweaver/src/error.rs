// Advanced error handling module for NetWeaver
// Provides rich error context for better debugging and user feedback

use thiserror::Error;
use std::net::Ipv4Addr;

/// Comprehensive error types for network operations
/// Each variant provides specific context about what went wrong and why
#[derive(Error, Debug)]
pub enum NetweaverError {
    /// Network connectivity errors
    #[error("Failed to connect to {host}:{port} - {reason}")]
    ConnectionFailed {
        host: String,
        port: u16,
        reason: String,
    },

    /// Host unreachable or not responding
    #[error("Host {ip} is unreachable after {attempts} attempts")]
    HostUnreachable {
        ip: Ipv4Addr,
        attempts: u32,
    },

    /// Port scanning errors
    #[error("Port scan failed for {ip} - {details}")]
    PortScanFailed {
        ip: Ipv4Addr,
        details: String,
    },

    /// DNS resolution failures
    #[error("Failed to resolve hostname '{hostname}' - {reason}")]
    DnsResolutionFailed {
        hostname: String,
        reason: String,
    },

    /// Packet crafting errors
    #[error("Failed to craft {packet_type} packet - {reason}")]
    PacketCraftFailed {
        packet_type: String,
        reason: String,
    },

    /// Packet parsing errors
    #[error("Failed to parse packet: {details}")]
    PacketParseFailed {
        details: String,
    },

    /// Socket operation errors
    #[error("Socket operation failed: {operation} - {reason}")]
    SocketError {
        operation: String,
        reason: String,
    },

    /// Permission/privilege errors
    #[error("Insufficient privileges for {operation}. Try running with sudo.")]
    PermissionDenied {
        operation: String,
    },

    /// Configuration errors
    #[error("Invalid configuration: {field} - {reason}")]
    ConfigError {
        field: String,
        reason: String,
    },

    /// Timeout errors
    #[error("Operation timed out after {duration_ms}ms: {operation}")]
    Timeout {
        operation: String,
        duration_ms: u64,
    },

    /// Resource exhaustion
    #[error("Resource limit exceeded: {resource} - {details}")]
    ResourceExhausted {
        resource: String,
        details: String,
    },

    /// Invalid input parameters
    #[error("Invalid parameter: {param} - {reason}")]
    InvalidParameter {
        param: String,
        reason: String,
    },

    /// File I/O errors
    #[error("File operation failed: {path} - {reason}")]
    FileError {
        path: String,
        reason: String,
    },

    /// Serialization/deserialization errors
    #[error("Failed to {operation} {format} data - {details}")]
    SerializationError {
        operation: String,
        format: String,
        details: String,
    },
}

/// Result type alias for NetWeaver operations
pub type Result<T> = std::result::Result<T, NetweaverError>;

/// Error context extension trait for enriching errors with additional info
pub trait ErrorContext<T> {
    fn with_host_context(self, host: &str, port: u16) -> Result<T>;
    fn with_ip_context(self, ip: Ipv4Addr) -> Result<T>;
    fn with_operation_context(self, operation: &str) -> Result<T>;
}

impl<T, E: std::fmt::Display> ErrorContext<T> for std::result::Result<T, E> {
    fn with_host_context(self, host: &str, port: u16) -> Result<T> {
        self.map_err(|e| NetweaverError::ConnectionFailed {
            host: host.to_string(),
            port,
            reason: e.to_string(),
        })
    }

    fn with_ip_context(self, ip: Ipv4Addr) -> Result<T> {
        self.map_err(|e| NetweaverError::PortScanFailed {
            ip,
            details: e.to_string(),
        })
    }

    fn with_operation_context(self, operation: &str) -> Result<T> {
        self.map_err(|e| NetweaverError::SocketError {
            operation: operation.to_string(),
            reason: e.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = NetweaverError::HostUnreachable {
            ip: "192.168.1.1".parse().unwrap(),
            attempts: 3,
        };
        let msg = err.to_string();
        assert!(msg.contains("192.168.1.1"));
        assert!(msg.contains("3 attempts"));
    }

    #[test]
    fn test_error_context() {
        let result: std::result::Result<(), &str> = Err("connection refused");
        let enriched = result.with_host_context("example.com", 80);
        assert!(enriched.is_err());
        
        let err_msg = enriched.unwrap_err().to_string();
        assert!(err_msg.contains("example.com"));
        assert!(err_msg.contains("80"));
    }
}
