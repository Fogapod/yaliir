use std::rc::Rc;

#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Link<T>,
}

type Link<T> = Option<Rc<Node<T>>>;

#[derive(Debug)]
pub struct LinkedList<T> {
    head: Link<T>,
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}
// pub struct IterMut<'a, T> {
//     next: Option<&'a mut Node<T>>,
// }
// pub struct IntoIter<T>(LinkedList<T>);

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn append(&self, data: T) -> LinkedList<T> {
        LinkedList {
            head: Some(Rc::new(Node {
                data: data,
                next: self.head.clone(),
            })),
        }
    }

    pub fn tail(&self) -> LinkedList<T> {
        LinkedList {
            head: self.head.as_ref().and_then(|node| node.next.clone()),
        }
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.data)
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.as_deref(),
        }
    }

    // pub fn iter_mut(&mut self) -> IterMut<T> {
    //     IterMut {
    //         next: self.head.as_deref_mut(),
    //     }
    // }

    // pub fn into_iter(self) -> IntoIter<T> {
    //     IntoIter(self)
    // }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut head = self.head.take();

        while let Some(node) = head {
            if let Ok(mut node) = Rc::try_unwrap(node) {
                head = node.next.take();
            } else {
                break;
            }
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.data
        })
    }
}

// impl<'a, T> Iterator for IterMut<'a, T> {
//     type Item = &'a mut T;

//     fn next(&mut self) -> Option<Self::Item> {
//         self.next.take().map(|node| {
//             self.next = node.next.as_deref_mut();
//             &mut node.data
//         })
//     }
// }
// impl<T> Iterator for IntoIter<T> {
//     type Item = T;

//     fn next(&mut self) -> Option<Self::Item> {
//         self.0.pop()
//     }
// }

#[cfg(test)]
mod test {
    use super::LinkedList;

    #[test]
    fn basic() {
        let list = LinkedList::new();
        assert_eq!(list.head(), None);

        let list = list.append(1).append(2).append(3);
        assert_eq!(list.head(), Some(&3));

        let list = list.tail();
        assert_eq!(list.head(), Some(&2));

        let list = list.tail();
        assert_eq!(list.head(), Some(&1));

        let list = list.tail();
        assert_eq!(list.head(), None);

        // Make sure empty tail works
        let list = list.tail();
        assert_eq!(list.head(), None);
    }

    #[test]
    fn iter() {
        let list = LinkedList::new().append(1).append(2);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
    }
}
