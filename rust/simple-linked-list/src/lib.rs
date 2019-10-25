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
 
impl<T: Clone> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList { head: None }
    }

    pub fn iter(&self) -> IterSimpleLinkedList<T> {
        IterSimpleLinkedList::new(self)
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
