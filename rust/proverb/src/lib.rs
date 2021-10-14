
const TEMPLATE: [&str; 2] = [
    "For want of a {} the {} was lost.",
    "And all for the want of a {}."
];

pub fn build_proverb(list: &[&str]) -> String {
    for word in list.iter() {
        if word == list.last() {
            return format!("And all for the want of a {}.", word);
        } else {
            return  format!("For want of a {} the {} was lost.", word, list.next());
        }
    }
 }
