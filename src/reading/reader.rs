use crate::books::{book::Book, ResultSelf};

use std::{
    cell::RefCell,
    fmt::{Debug, Formatter, Result},
    rc::{Rc, Weak},
};

/// Reader structure, which contains
/// name, family, father, age, ~~simple~~ books he' d read
/// and book which he is reading now (or None)

pub(crate) struct Reader {
    pub(crate) name: String,
    pub(crate) family: String,
    pub(crate) father: String,
    pub(crate) age: u8,
    pub(crate) books: Vec<Weak<RefCell<Book>>>,
    pub(crate) reading: Option<Weak<RefCell<Book>>>,
}

impl Drop for Reader {
    /// Destructor for Reader.
    /// It's used to debug code

    #[inline]
    fn drop(&mut self) {
        println!(
            "Readers {} {} {} is deleted",
            self.name, self.family, self.father
        );
    }
}

impl Debug for Reader {
    /// Print for Reader.
    /// It's used to debug code

    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("Reader")
            .field("name", &self.name)
            .field("family", &self.family)
            .field("father", &self.father)
            .field("age", &self.age)
            .field(
                "books",
                &self
                    .books
                    .iter()
                    .map(|x| {
                        (*(*x).upgrade().unwrap()).borrow().title().to_string()
                            + " "
                            + (*(*x).upgrade().unwrap()).borrow().author().as_str()
                            + " "
                            + format!("{}", (*(*x).upgrade().unwrap()).borrow().pages()).as_str()
                    })
                    .collect::<Vec<_>>(),
            )
            .finish()
    }
}

impl PartialEq for Reader {
    /// Compare Reader by == / !=
    /// by their params

    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.family == other.family
            && self.father == other.father
            && self.age == other.age
    }
}

/// Compare Reader by == / !=
/// by their params

impl Eq for Reader {}

impl Clone for Reader {
    /// Clones reader
    /// Clears all books pointer.
    /// After that Book System also **MUST** be cloned

    #[inline]
    fn clone(&self) -> Self {
        Reader {
            name: self.name.clone(),
            family: self.family.clone(),
            father: self.father.clone(),
            age: self.age,
            books: vec![],
            reading: None,
        }
    }

    /// Clones reader from another reader

    #[inline]
    fn clone_from(&mut self, other: &Self) {
        *self = other.clone()
    }
}

impl Reader {
    /// Creates new Reader with chosen
    /// 1-st name, 2-nd name, mid. name and age.
    /// It has no books

    #[inline]
    pub(crate) const fn new(
        new_name: String,
        new_family: String,
        new_father: String,
        new_age: u8,
    ) -> Self {
        Reader {
            name: new_name,
            family: new_family,
            father: new_father,
            age: new_age,
            books: vec![],
            reading: None,
        }
    }

    /// Find book by smart pointer.
    /// If ok, returns index of the first occurrence,
    /// else none

    #[inline]
    pub(crate) fn find_book_first(&self, book: &Rc<RefCell<Book>>) -> Option<usize> {
        self.books
            .iter()
            .position(|x| x.upgrade().unwrap().as_ptr() == book.as_ptr())
    }

    /// Find book by smart pointer.
    /// If ok, returns index of the last occurrence,
    /// else none

    #[inline]
    pub(crate) fn find_book_last(&self, book: &Rc<RefCell<Book>>) -> Option<usize> {
        self.books
            .iter()
            .rev()
            .position(|x| x.upgrade().unwrap().as_ptr() == book.as_ptr())
    }

    /// Function, that uses after giving book to reader.
    /// Adds book to books and reading params

    #[inline]
    pub(crate) fn start_reading(&mut self, book: &Rc<RefCell<Book>>) -> &mut Self {
        self.books.push(Rc::downgrade(&book));
        self.reading = Some(Rc::downgrade(&book));
        self
    }

    /// Function, that uses after giving book to reader.
    /// Sets reading param as None

    #[inline]
    pub(crate) fn finish_reading(&mut self) {
        self.reading = None;
    }

    /// Removes book

    #[inline]
    pub(crate) fn remove_book(&mut self, book: &mut Book) -> &mut Self {
        if book.is_using
            && (*(book.readers.last().unwrap().0).upgrade().unwrap()).as_ptr()
                == self as *mut Reader
        {
            self.reading = None;
            book.is_using = false;
        }

        self.books = self
            .books
            .clone()
            .into_iter()
            .filter(|x| (*(*x).upgrade().unwrap()).as_ptr() != book as *mut Book)
            .collect();

        self
    }

    /// Removes all simple books.
    /// Used to delete reader

    #[inline]
    pub(crate) fn remove_all_books(&mut self) -> &mut Self {
        while !self.books.is_empty() {
            if (*self.books.last().unwrap().upgrade().unwrap())
                .borrow()
                .is_using
                && (*((*self.books.last().unwrap().upgrade().unwrap())
                    .borrow()
                    .readers
                    .last()
                    .unwrap()
                    .0)
                    .upgrade()
                    .unwrap())
                .as_ptr()
                    == self as *mut Reader
            {
                (*self.books.last().unwrap().upgrade().unwrap())
                    .borrow_mut()
                    .is_using = false;
            }

            (*self.books.last().unwrap().upgrade().unwrap())
                .borrow_mut()
                .remove_reader(self);
            self.books.pop();
        }
        self
    }

    /// Changes reader's name

    #[inline]
    pub(crate) fn change_name(&mut self, new_name: String) -> ResultSelf<Self> {
        return if new_name.is_empty() {
            Err(0)
        } else {
            self.name = new_name;
            Ok(self)
        };
    }

    /// Changes reader's 2-nd name

    #[inline]
    pub(crate) fn change_family(&mut self, new_family: String) -> ResultSelf<Self> {
        return if new_family.is_empty() {
            Err(0)
        } else {
            self.family = new_family;
            Ok(self)
        };
    }

    /// Changes reader's mid. name

    #[inline]
    pub(crate) fn change_father(&mut self, new_father: String) -> ResultSelf<Self> {
        return if new_father.is_empty() {
            Err(0)
        } else {
            self.father = new_father;
            Ok(self)
        };
    }

    /// Changes reader's age

    #[inline]
    pub(crate) fn change_age(&mut self, new_age: u8) -> &mut Self {
        self.age = new_age;
        self
    }
}
