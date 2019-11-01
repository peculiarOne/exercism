pub struct SimpleLinkedList<T> {
    head: Option<Elem<T>>,
}

pub struct Elem<T> {
    value: T,
    next_elem: Option<Box<Elem<T>>>,
}

impl<T> Elem<T> {
    pub fn new(_value: T, next: Option<Box<Elem<T>>>) -> Self {
        Elem {
            value: _value,
            next_elem: next,
        }
    }
}

pub struct IterSimpleLinkedList<'a, T: 'a> {
    list: &'a SimpleLinkedList<T>,
    current: Option<Elem<T>>,
}
 impl<'a, T: 'a + Clone> IterSimpleLinkedList<'a, T> {
     fn new(list: &'a SimpleLinkedList<T>) -> Self {
         Self {
             list: list,
             current: list.head
         }
     }
 }

impl<'a, T: 'a + Clone> Iterator for IterSimpleLinkedList<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        // update next
        let prev = self.current.map(|e| e.value);
            let next_current = match self.current {
                None => None,
                Some(x) => x.next_elem.map(|v| *v)
            };

        self.current = next_current;

        prev

    }
}
 
impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList { head: None }
    }

    pub fn len(&self) -> usize {
        fn len_elem<T>(elem: &Option<Box<Elem<T>>>) -> usize {
            match elem {
                Some(e) => 1 + len_elem(&e.next_elem),
                None => 0,
            }
        }

        match &self.head {
            Some(x) => 1 + len_elem(&x.next_elem),
            None => 0,
        }
    }

    pub fn push(&mut self, _element: T) {
        let new_head = Elem::new(_element, self.head.take().map(Box::new));
        self.head = Some(new_head);
    }

    pub fn pop(&mut self) -> Option<T> {
        let head = self.head.take();
        match head {
            Some(mut h) => {
                // map(|x| *x) dereferences our Box turning a Box<Elem<T>> into an Elem<T>
                let v = h.next_elem.take().map(|x| *x);
                self.head = v;
                Some(h.value)
                },
            None => None,
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|e| &e.value)
    }

        pub fn rev(self) -> SimpleLinkedList<T> {
        unimplemented!()
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        unimplemented!()
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
        unimplemented!()
    }
}