module TwoFer

open System

let twoFer (input: string option): string =
    let target =
        match input with
        | Some(input) -> input
        | None -> "you"

    String.Format("One for {0}, one for me.", target)
