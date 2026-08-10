pub trait ServerMetrics: Send + Sync {
    fn new_client(&self, run_id: &str);
    fn close_client(&self);

    fn new_proxy(&self, name: &str, proxy_type: &str, user: &str, client_id: &str);
    fn close_proxy(&self, name: &str, proxy_type: &str);

    fn open_connection(&self, name: &str, proxy_type: &str);
    fn close_connection(&self, name: &str, proxy_type: &str);

    fn add_traffic_in(&self, name: &str, proxy_type: &str, bytes: u64);
    fn add_traffic_out(&self, name: &str, proxy_type: &str, bytes: u64);
}
