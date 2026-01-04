fn main() {
    let my_name = "Loki Laufeyson";

    assert!(my_name == "Loki Laufeyson", "not good");
    assert_eq!(my_name, "Loki Laufeyson", "not my_name");
    assert_ne!(my_name, "Mithridates", "that's my_name");
}
