pub struct PulseMethodEntry {
    pub name: &'static str,
    pub apply: fn(&str, &str) -> Result<String, String>,
}

inventory::collect!(PulseMethodEntry);

pub fn find(name: &str) -> Option<&'static PulseMethodEntry> {
    let snake = crate::bindings::name_norm::to_snake_case(name);
    inventory::iter::<PulseMethodEntry>().find(|e| e.name == name || e.name == snake)
}

pub fn invoke(html: &str, name: &str, args_json: &str) -> Result<String, String> {
    match find(name) {
        Some(entry) => (entry.apply)(html, args_json),
        None => Err(format!("seraplot: unknown pulse method '{name}'")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ok_echo(_html: &str, args: &str) -> Result<String, String> {
        Ok(args.to_string())
    }

    inventory::submit! { PulseMethodEntry { name: "test_pulse_echo_method", apply: ok_echo } }

    #[test]
    fn find_resolves_a_registered_entry_by_exact_name() {
        assert!(find("test_pulse_echo_method").is_some());
    }

    #[test]
    fn find_resolves_a_registered_entry_by_pascal_case_name() {
        assert!(find("TestPulseEchoMethod").is_some());
    }

    #[test]
    fn find_returns_none_for_an_unregistered_name() {
        assert!(find("not_a_real_pulse_method").is_none());
    }

    #[test]
    fn invoke_returns_the_entrys_result_on_a_match() {
        assert_eq!(invoke("<html/>", "test_pulse_echo_method", "{}").unwrap(), "{}");
    }

    #[test]
    fn invoke_returns_an_error_for_an_unregistered_name() {
        assert!(invoke("<html/>", "not_a_real_pulse_method", "{}").is_err());
    }
}
