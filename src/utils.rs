use rand::Rng;

// Base58 character set (excludes 0, O, I, l to avoid ambiguity)
const BASE58_CHARSET: &[u8] = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

/// Generate a random Solana address (base58-like string)
pub fn generate_random_address() -> String {
    let mut rng = rand::thread_rng();
    
    (0..44)
        .map(|_| {
            let idx = rng.gen_range(0..BASE58_CHARSET.len());
            BASE58_CHARSET[idx] as char
        })
        .collect()
}

/// Generate a random transaction signature
pub fn generate_random_signature() -> String {
    let mut rng = rand::thread_rng();
    
    (0..88)
        .map(|_| {
            let idx = rng.gen_range(0..BASE58_CHARSET.len());
            BASE58_CHARSET[idx] as char
        })
        .collect()
}

/// Generate a random floating point number in range
pub fn random_f64(min: f64, max: f64) -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}

/// Generate a random boolean with given probability
pub fn random_bool(probability: f64) -> bool {
    let mut rng = rand::thread_rng();
    rng.gen_bool(probability)
}

/// Format SOL amount with proper decimals
pub fn format_sol(amount: f64) -> String {
    format!("{:.4} SOL", amount)
}

/// Format basis points as percentage
pub fn format_bps(bps: u16) -> String {
    let percentage = bps as f64 / 100.0;
    format!("{:.2}%", percentage)
}

/// Calculate percentage change
pub fn calculate_percentage_change(old: f64, new: f64) -> f64 {
    if old == 0.0 {
        return 0.0;
    }
    ((new - old) / old) * 100.0
}

/// Validate Solana address format (basic check)
/// Solana addresses are 44 characters of base58 (excludes 0, O, I, l)
pub fn is_valid_address(address: &str) -> bool {
    if address.len() != 44 {
        return false;
    }
    
    // Check all characters are valid base58
    address.chars().all(|c| {
        matches!(c, '1'..='9' | 'A'..='H' | 'J'..='N' | 'P'..='Z' | 'a'..='k' | 'm'..='z')
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_generate_random_address() {
        let address = generate_random_address();
        assert_eq!(address.len(), 44);
        assert!(address.chars().all(|c| c.is_alphanumeric()));
    }
    
    #[test]
    fn test_generate_random_signature() {
        let signature = generate_random_signature();
        assert_eq!(signature.len(), 88);
        assert!(signature.chars().all(|c| c.is_alphanumeric()));
    }
    
    #[test]
    fn test_random_f64() {
        let value = random_f64(1.0, 10.0);
        assert!(value >= 1.0 && value <= 10.0);
    }
    
    #[test]
    fn test_random_bool() {
        // Test with 100% probability
        assert!(random_bool(1.0));
        
        // Test with 0% probability
        assert!(!random_bool(0.0));
    }
    
    #[test]
    fn test_format_sol() {
        assert_eq!(format_sol(1.5), "1.5000 SOL");
        assert_eq!(format_sol(0.1234567), "0.1235 SOL");
    }
    
    #[test]
    fn test_format_bps() {
        assert_eq!(format_bps(50), "0.50%");
        assert_eq!(format_bps(100), "1.00%");
    }
    
    #[test]
    fn test_calculate_percentage_change() {
        assert_eq!(calculate_percentage_change(100.0, 150.0), 50.0);
        assert_eq!(calculate_percentage_change(100.0, 50.0), -50.0);
        assert_eq!(calculate_percentage_change(0.0, 100.0), 0.0);
    }
    
    #[test]
    fn test_is_valid_address() {
        let valid = generate_random_address();
        assert!(is_valid_address(&valid));
        
        assert!(!is_valid_address("too_short"));
        assert!(!is_valid_address("this_is_way_too_long_to_be_a_valid_solana_address"));
        
        // Test invalid base58 characters (0, O, I, l)
        let invalid_with_zero = format!("0{}", "1".repeat(43));
        assert!(!is_valid_address(&invalid_with_zero));
        
        let invalid_with_O = format!("O{}", "1".repeat(43));
        assert!(!is_valid_address(&invalid_with_O));
        
        let invalid_with_I = format!("I{}", "1".repeat(43));
        assert!(!is_valid_address(&invalid_with_I));
        
        let invalid_with_l = format!("l{}", "1".repeat(43));
        assert!(!is_valid_address(&invalid_with_l));
    }
}
