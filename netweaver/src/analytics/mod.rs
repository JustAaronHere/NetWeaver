use std::collections::VecDeque;
use std::time::{Duration, Instant};

/// Statistical analysis module for network performance metrics
/// Provides real-time and historical analysis of latency, bandwidth, and packet loss

/// Latency analyzer with sliding window statistics
/// Tracks latency samples over time and provides statistical analysis including
/// average, min, max, jitter, and anomaly detection using standard deviation
#[derive(Debug, Clone)]
pub struct LatencyAnalyzer {
    samples: VecDeque<f64>,
    max_samples: usize,
}

impl LatencyAnalyzer {
    pub fn new(max_samples: usize) -> Self {
        Self {
            samples: VecDeque::with_capacity(max_samples),
            max_samples,
        }
    }
    
    pub fn add_sample(&mut self, latency_ms: f64) {
        if self.samples.len() >= self.max_samples {
            self.samples.pop_front();
        }
        self.samples.push_back(latency_ms);
    }
    
    pub fn average(&self) -> f64 {
        if self.samples.is_empty() {
            return 0.0;
        }
        self.samples.iter().sum::<f64>() / self.samples.len() as f64
    }
    
    pub fn min(&self) -> f64 {
        self.samples.iter().cloned().fold(f64::INFINITY, f64::min)
    }
    
    pub fn max(&self) -> f64 {
        self.samples.iter().cloned().fold(f64::NEG_INFINITY, f64::max)
    }
    
    /// Calculate jitter (variance in latency) using RFC 3550 algorithm
    /// Jitter represents the variation in packet arrival times and is critical
    /// for real-time applications like VoIP and video streaming
    pub fn jitter(&self) -> f64 {
        if self.samples.len() < 2 {
            return 0.0;
        }
        
        let mut sum = 0.0;
        for i in 1..self.samples.len() {
            sum += (self.samples[i] - self.samples[i - 1]).abs();
        }
        sum / (self.samples.len() - 1) as f64
    }
    
    /// Detect latency anomalies using statistical analysis
    /// Uses z-score method: flags values beyond threshold * std_dev from mean
    /// Typical threshold: 3.0 for 99.7% confidence interval
    pub fn detect_anomaly(&self, threshold_std_dev: f64) -> bool {
        if self.samples.len() < 10 {
            return false;
        }
        
        let avg = self.average();
        let variance: f64 = self.samples.iter()
            .map(|&x| (x - avg).powi(2))
            .sum::<f64>() / self.samples.len() as f64;
        let std_dev = variance.sqrt();
        
        self.samples.back()
            .map(|&last| (last - avg).abs() > threshold_std_dev * std_dev)
            .unwrap_or(false)
    }
}

/// Bandwidth analyzer with sliding time window
/// Tracks data transfer over time to calculate current bandwidth utilization
/// Uses a moving window to provide real-time bandwidth measurements
#[derive(Debug, Clone)]
pub struct BandwidthAnalyzer {
    bytes_history: VecDeque<(Instant, u64)>,
    window: Duration,
}

impl BandwidthAnalyzer {
    pub fn new(window: Duration) -> Self {
        Self {
            bytes_history: VecDeque::new(),
            window,
        }
    }
    
    pub fn add_measurement(&mut self, bytes: u64) {
        let now = Instant::now();
        self.bytes_history.push_back((now, bytes));
        
        let cutoff = now - self.window;
        while let Some(&(time, _)) = self.bytes_history.front() {
            if time < cutoff {
                self.bytes_history.pop_front();
            } else {
                break;
            }
        }
    }
    
    /// Calculate current bandwidth in bytes per second
    /// Returns the average bandwidth over the configured time window
    pub fn current_bandwidth(&self) -> f64 {
        if self.bytes_history.len() < 2 {
            return 0.0;
        }
        
        let total_bytes: u64 = self.bytes_history.iter().map(|(_, b)| b).sum();
        let duration = self.bytes_history.back().unwrap().0
            .duration_since(self.bytes_history.front().unwrap().0)
            .as_secs_f64();
        
        if duration > 0.0 {
            total_bytes as f64 / duration
        } else {
            0.0
        }
    }
}

#[derive(Debug, Clone)]
pub struct PacketLossDetector {
    sent: u64,
    received: u64,
}

impl PacketLossDetector {
    pub fn new() -> Self {
        Self {
            sent: 0,
            received: 0,
        }
    }
    
    pub fn record_sent(&mut self, count: u64) {
        self.sent += count;
    }
    
    pub fn record_received(&mut self, count: u64) {
        self.received += count;
    }
    
    pub fn loss_rate(&self) -> f64 {
        if self.sent == 0 {
            return 0.0;
        }
        
        let lost = self.sent.saturating_sub(self.received);
        (lost as f64 / self.sent as f64) * 100.0
    }
    
    pub fn reset(&mut self) {
        self.sent = 0;
        self.received = 0;
    }
}

impl Default for PacketLossDetector {
    fn default() -> Self {
        Self::new()
    }
}
