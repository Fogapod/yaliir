#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
pub struct LinkedList<T> {
    head: Link<T>,
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}
pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}
pub struct IntoIter<T>(LinkedList<T>);

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn push(&mut self, data: T) {
        let node = Node {
            data: data,
            next: self.head.take(),
        };

        self.head = Some(Box::new(node));
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.data
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.data)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.data)
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.as_deref(),
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            next: self.head.as_deref_mut(),
        }
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn len(&self) -> usize {
        self.iter().count()
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();

        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
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

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.data
        })
    }
}
impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

#[cfg(test)]
mod test {

    use super::LinkedList;

    #[test]
    fn push_pop_len() {
        let mut list = LinkedList::new();

        assert_eq!(list.len(), 0);

        let data1 = "hello";
        let data2 = "world";
        let data3 = "foobar";

        list.push(data1);

        assert_eq!(list.len(), 1);

        list.push(data2);
        list.push(data3);

        list.pop();
        assert_eq!(list.len(), 2);

        assert_eq!(list.pop(), Some(data2));

        list.pop();
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn peek() {
        let mut list = LinkedList::new();

        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);

        list.push(1);
        assert_eq!(list.peek(), Some(&1));
        assert_eq!(list.peek_mut(), Some(&mut 1));

        list.push(2);
        assert_eq!(list.peek(), Some(&2));
        assert_eq!(list.peek_mut(), Some(&mut 2));

        list.pop();
        assert_eq!(list.peek(), Some(&1));
        assert_eq!(list.peek_mut(), Some(&mut 1));

        list.peek_mut().map(|value| *value = 42);

        assert_eq!(list.peek(), Some(&42));
        assert_eq!(list.peek_mut(), Some(&mut 42));

        assert_eq!(list.pop(), Some(42));
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);
    }

    #[test]
    fn iter() {
        let mut list = LinkedList::new();
        list.push(1);
        list.push(2);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter_mut() {
        let mut list = LinkedList::new();
        list.push(1);
        list.push(2);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn into_iter() {
        let mut list = LinkedList::new();
        list.push(1);
        list.push(2);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }
}
