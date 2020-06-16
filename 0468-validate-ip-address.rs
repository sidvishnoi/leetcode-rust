struct Solution;

const NEITHER: &'static str = "Neither";

impl Solution {
    pub fn valid_ip_address(ip: String) -> String {
        if ip.matches('.').count() == 3 {
            Self::valid_ipv4_address(&ip).to_string()
        } else if ip.matches(':').count() == 7 {
            Self::valid_ipv6_address(&ip).to_string()
        } else {
            NEITHER.to_string()
        }
    }

    fn valid_ipv4_address(ip: &str) -> &'static str {
        for num in ip.split('.') {
            if num.len() == 0 || num.len() > 3 {
                return NEITHER;
            }
            if num.as_bytes()[0] == b'0' && num.len() != 1 {
                return NEITHER;
            }
            if num.parse::<u8>().is_err() {
                return NEITHER;
            }
        }
        "IPv4"
    }

    fn valid_ipv6_address(ip: &str) -> &'static str {
        for num in ip.split(':') {
            if num.len() == 0 || num.len() > 4 {
                return NEITHER;
            }
            if num.as_bytes().iter().any(|x| !x.is_ascii_hexdigit()) {
                return NEITHER;
            }
        }
        "IPv6"
    }
}

fn main() {
    assert_eq!(
        "IPv4",
        Solution::valid_ip_address("172.16.254.1".to_string())
    );
    assert_eq!(
        "IPv6",
        Solution::valid_ip_address("2001:0db8:85a3:0:0:8A2E:0370:7334".to_string())
    );
    assert_eq!(
        "Neither",
        Solution::valid_ip_address("256.256.256.256".to_string())
    );
    assert_eq!(
        "Neither",
        Solution::valid_ip_address("2001:0db8:85g3:0:0:8A2E:0370:7334".to_string())
    );
    assert_eq!(
        "Neither",
        Solution::valid_ip_address("0.0.0.-0".to_string())
    );
    assert_eq!(
        "Neither",
        Solution::valid_ip_address("0.0.0.1-0".to_string())
    );
}
