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

pub struct SimpleLinkedListIter<'a, T> {
    list: &'a SimpleLinkedList<T>,
    current: &'a Option<Elem<T>>
}

impl<'a, T> Iterator for SimpleLinkedListIter<'a, T> {
    type Item = &'a Elem<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let r = match self.current {
            None => None,
            Some(c) => {
                self.current = match &c.next_elem {
                    None => None,
                    Some(x) => Some(x)
                }.as_ref();

                Some(c)
            }
        };
        r
    }
}
 
impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList { head: None }
    }

    pub fn iter(&self) -> SimpleLinkedListIter<T> {
        SimpleLinkedListIter {
            list: self,
            current: &self.head
        }
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
}

impl<T: Clone> SimpleLinkedList<T> {
    pub fn rev(&self) -> SimpleLinkedList<T> {
        // let mut reversed = SimpleLinkedList::new();
        // fn reverse<T: Clone>(reversed: &mut SimpleLinkedList<T>, current: &Option<Elem<T>>) {
        //     if let Some(element) = current {
        //         reversed.push(element.value.clone());
        //         reverse(reversed, element.next_elem.map(|x| *x));
        //     }
        // }
        // reverse(&mut reversed, self.head);
        unimplemented!()
    }
}

impl<'a, T: Clone> From<&'a [T]> for SimpleLinkedList<T> {
    fn from(_item: &[T]) -> Self {
        let mut list = SimpleLinkedList::new();
        for it in _item.iter() {
            list.push(it.clone())
        };
        list
    }
}

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(mut self) -> Vec<T> {
        let mut vec = vec![];
        while let Some(element) = self.pop() {
            vec.push(element)
        }
        vec.reverse();
        vec
    }
}
