module ErrorHandling

open System

let handleErrorByThrowingException () = failwith "test"

let handleErrorByReturningOption (input: string): Option<int> =
    match Int32.TryParse input with
    | (true, n) -> Some n
    | (false, _) -> None

let handleErrorByReturningResult (input: string) =
    match Int32.TryParse input with
    | (true, n) -> Ok n
    | (false, _) -> Error "Could not convert input to integer"

let bind (switchFunction: int -> Result<int, string>) = function
    | Ok x -> switchFunction x
    | Error e -> Error e

let cleanupDisposablesWhenThrowingException (resource: IDisposable) =
    try
        failwith "test"
    finally
        resource.Dispose()
