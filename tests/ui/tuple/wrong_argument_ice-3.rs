struct Process;

pub type Group = (Vec<String>, Vec<Process>);

fn test(process: &Process, groups: Vec<Group>) -> Vec<Group> {
    let new_group = vec![String::new()];

    if groups.capacity() == 0 {
        groups.push(new_group, vec![process]);
        //~^ ERROR this method takes 1 argument but 2 arguments were supplied
        return groups;
    }

    todo!()
}

fn main() {}

// ferrocene-annotations: fls_qcb1n9c0e5hz
// Functions
// ferrocene-annotations: fls_kgvleup5mdhq
// Type Aliases
// ferrocene-annotations: fls_4ckl3n2ko3i4
// Tuple Types
// ferrocene-annotations: fls_z7q8kbjwdc7g
// Method Call Expressions
//
// ferrocene-annotations: fls_wqazkzle0ix9
// Method Resolution
