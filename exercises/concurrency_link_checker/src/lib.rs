use std::sync::mpsc;
use std::thread;

pub fn check_links(links: Vec<&'static str>) -> Vec<(&'static str, bool)> {
    let (tx, rx) = mpsc::channel();

    for link in links {
        let t_tx = tx.clone();
        thread::spawn(move || {
            let is_valid = link.starts_with("https://");
            t_tx.send((link, is_valid)).unwrap();
        });
    }
    drop(tx);

    rx.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_links() {
        let links = vec!["https://rust-lang.org", "http://insecure.site"];
        let results = check_links(links);
        assert_eq!(results.len(), 2);
    }
}
