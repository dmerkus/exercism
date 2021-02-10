module BinarySearchTree

type Tree<'T> =
    { Data: 'T
      Left: Option<Tree<'T>>
      Right: Option<Tree<'T>> }

let leaf x =
    { Data = x
      Left = None
      Right = None }

let left (node: Tree<'T>) = node.Left

let right (node: Tree<'T>) = node.Right

let data (node: Tree<'T>) = node.Data

let rec insert (root: Tree<'T>) (data: 'T) =
    match (root, data <= root.Data) with
    | ({ Left = None }, true) -> { root with Left = Some(leaf data) }
    | ({ Right = None }, false) -> { root with Right = Some(leaf data) }
    | ({ Left = Some(left) }, true) -> { root with Left = Some(insert left data) }
    | ({ Right = Some(right) }, false) -> { root with Right = Some(insert right data) }

let rec create (items: list<'T>): Tree<'T> =
    match items with
    | [] -> failwith "At least one item is required to create the search tree."
    | [ head ] -> leaf head
    | head :: tail -> List.fold insert (create [ head ]) tail

let rec sortedData (tree: Tree<'T>): list<'T> =
    let rec treeToList (tree: Option<Tree<'T>>): list<'T> =
        match tree with
        | Some tree -> (treeToList tree.Left) @ [ tree.Data ] @ (treeToList tree.Right)
        | None -> []

    treeToList (Some tree)
