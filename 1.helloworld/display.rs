use std::fmt;

struct Structure(!32);

impl fmt::Display for Structure {
    fn fnt($self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }


}




