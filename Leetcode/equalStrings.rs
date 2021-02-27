

// DEFANG IP ADDRESS

// Given a valid (IPv4) IP address, return a defanged version of that IP address.
// A defanged IP address replaces every period "." with "[.]".

// Input: address = "255.100.50.0"
// Output: "255[.]100[.]50[.]0"


fn main() {
    defang_i_p("1.1.1.1".to_string());
}

fn defang_i_p(address: String) -> String {
    return address.replace(".", "[.]");
}

