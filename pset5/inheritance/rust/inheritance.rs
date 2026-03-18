use std::os::raw::{c_int, c_char};

mod cs50;

extern "C" {
    fn random_bit() -> c_int;
    fn random_allele() -> c_char;
}

#[repr(C)]
#[derive(Clone, Copy)]
#[allow(non_snake_case)]
pub struct person {
    pub parents: [*mut person; 2],
    pub alleles: [u8; 2],
}

pub struct SafePerson {
    pub parents: [Option<Box<SafePerson>>; 2],
    pub alleles: [u8; 2],
}

impl SafePerson {
    pub fn into_raw(self) -> *mut person {
        // convert parents recursively
        let parents: [*mut person; 2] = self.parents.map(|parent| {
            match parent {
                Some(p) => p.into_raw(),
                None => std::ptr::null_mut(),
            }
        });

        let p = person {
            parents,
            alleles: self.alleles,
        };

        Box::into_raw(Box::new(p))  
    }
}

#[no_mangle]
pub unsafe extern "C" fn create_family_rs(generations: c_int) -> *mut person {
    let family = create_family(generations);
    family.into_raw()
}

fn create_family(generations: i32) -> SafePerson {
    // TODO: Allocate memory for new person
    let mut person = SafePerson {
        parents: [None, None],
        alleles: [0; 2],
    };

    // If there are still generations left to create
    if generations > 1
    {
        // Create two new parents for current person by recursively calling create_family
        let parent0 = create_family(generations - 1);
        let parent1 = create_family(generations - 1);

        // Set parent pointers for current person
        person.parents[0] = Some(Box::new(parent0));
        person.parents[1] = Some(Box::new(parent1));

        // Randomly assign current person's alleles based on the alleles of their parents
        let choice0 = unsafe { random_bit() as usize }; 
        let choice1 = unsafe { random_bit() as usize };

        person.alleles[0] = person.parents[0].as_ref().unwrap().alleles[choice0];
        person.alleles[1] = person.parents[1].as_ref().unwrap().alleles[choice1];
    }

    // If there are no generations left to create
    else
    {
        // Set parent pointers to NULL
        person.parents[0] = None;
        person.parents[1] = None;

        // Randomly assign alleles
        person.alleles[0] = unsafe { random_allele() as u8 };
        person.alleles[1] = unsafe { random_allele() as u8 };
    }

    // Return newly created person
    person
}

#[no_mangle]
pub unsafe extern "C" fn free_family_rs(p: *mut person) {
    // Handle base case
    if p.is_null() {
        return;
    }

    // Free parents recursively
    let boxed = Box::from_raw(p);

    for &parent in &boxed.parents {
        // Free child
        free_family_rs(parent);
    }
}