pub struct List<T> {
    head: Link<T>
}

struct Node<T> {
    elem: T,
    next: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }
    pub fn push(&mut self, n: T) {
        let new_node = Node {
            elem: n,
            next: self.head.take(),
        };

        self.head = Link::Some(Box::new(new_node))
    }
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }
    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.elem
        })
    }
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| {
            &mut node.elem
        })
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();
        assert_eq!(list.pop(), None);
        list.push(1);
        assert_eq!(list.pop(), Some(1));
        list.push(2);
        list.push(3);
        list.push(4);
        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn peek() {
        let mut list = List::new();
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);
        list.push(2);
        list.push(3);
        list.push(4);
        assert_eq!(list.peek(), Some(&4));
        assert_eq!(list.peek_mut(), Some(&mut 4));
        list.peek_mut().map(|v| {
            *v = 42
        });
        assert_eq!(list.peek(), Some(&42));
        assert_eq!(list.pop(), Some(42));
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push(2);
        list.push(3);
        list.push(4);
        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(4));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), None);
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Link::Some(mut boxed_node) = cur_link {
            cur_link = self.head.take();
        }
    }
}

/// IntoIter consumes the collection
pub struct IntoIter<T> {
    list: List<T>,
}

/// into_iter takes a self and returns IntoIter
impl<T> List<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter {list: self}
    }
}

/// next() can simply call self.list.pop()
impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) ->Option<T> {
        self.list.pop()
    }
}

/// Iter struct - next is an Option of a ref Node T
/// T needs a lifetime to use on Iterator::Item
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>
}

impl<T> List<T> {
    /// return an Iter struct with next set to ref of first Node if exists
    pub fn iter(&self) -> Iter<T> {
        Iter { next: self.head.as_ref().map(|node| {
            &**node
        })}
    }
}

/// needs a lifetime for Item
/// fn next() takes a mut ref to self and returns a Option of ref Self Item
/// to reassign self.next map over node.next as a ref and deref twice
/// retrun a ref to node.elem
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|node| { &**node});
            &node.elem
        })
    }
}


#[test]
fn iter() {
    let mut list = List::new();
    list.push(1);
    list.push(2);
    list.push(3);

    let mut iter = list.iter();
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&1));
}

/// Iter struct - next is an Option of a mut ref Node T
/// T needs a lifetime to use on Iterator::Item
pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>
}

impl<T> List<T> {
    /// return an Iter struct with next set to mut ref of first Node if exists
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut { next: self.head.as_mut().map(|node| {
            &mut **node
        })}
    }
}

/// needs a lifetime for Item
/// fn next() takes a mut ref to self and returns a Option of mut ref Self Item
/// to reassign self.next take() from the option (replaves the T with None),
/// map over the value as a mut ref and deref twice
/// return a mut ref to node.elem
///
impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_mut().map(|node| { &mut **node});
            &mut node.elem
        })
    }
}

#[test]
fn iter_mut() {
    let mut list = List::new();
    list.push(1); list.push(2); list.push(3);

    let mut iter = list.iter_mut();
    assert_eq!(iter.next(), Some(&mut 3));
    assert_eq!(iter.next(), Some(&mut 2));
    assert_eq!(iter.next(), Some(&mut 1));
}