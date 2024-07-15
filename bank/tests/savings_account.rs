use bank::SavingsAccounts;

#[test]
fn should_have_a_starting_balance_of_0() {
    let account = SavingsAccounts::new(0);
    assert_eq!(account.get_balance(), 0);
}
