use rand::{thread_rng, Rng};
pub use std::{
    self,
    cmp::{
        self,
        Eq::{self},
        Ordering::{
            self,
            Equal::{self},
            Greater::{self},
            Less::{self},
        },
        PartialEq::{self},
        PartialOrd::{self},
    },
    collections::{
        self,
        binary_heap::{
            self,
            Drain::{self},
            PeekMut::{self},
        },
        btree_map::{
            self,
            IntoKeys::{self},
            IterMut::{self},
        },
        btree_map::{
            IntoIter::{self},
            OccupiedEntry::{self},
            Values::{self},
            ValuesMut::{self},
        },
        BTreeMap::{self},
        BTreeSet::{self as dakll},
        LinkedList::{self as LinkedList11},
        VecDeque::{self},
    },
    default::{self, Default},
    fmt::{
        self as lklak,
        format::{self as lkflk},
        Arguments::{self},
        Result as fmtResult,
        Write::{self},
    },
    io::{
        self,
        prelude::{
            self,
            Read::{self},
            Seek::{self},
        },
        ErrorKind::{
            self as kjdakjcak, AlreadyExists,
            PermissionDenied::{self},
            WriteZero::{self},
        },
        Result as ioResult,
    },
    os::{
        self,
        windows::{
            fs::{
                symlink_file::{self},
                FileExt::{self},
                OpenOptionsExt::{self as ladll},
            },
            io::{
                AsRawHandle::{self},
                AsRawSocket::{self},
            },
        },
    },
};

fn main() {
    let secret_number = thread_rng().gen_range(1, 101);

    println!("dajklasl");
}

mod front_of_house;

mod back_of_house;

pub use crate::front_of_house::hosting;

use self::front_of_house::serving;

pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist();

    front_of_house::hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("rye bread");

    meal.toast = String::from("wheat bread");

    println!("get {} toast!", meal.toast);

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    hosting::add_to_waitlist();

    serving::serve_order();
}
