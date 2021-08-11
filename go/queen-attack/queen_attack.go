package queenattack

import (
	"queenattack/chess"
)

func Abs(a byte, b byte) byte {
	if a > b {
		return a - b
	} else {
		return b - a
	}
}

type IllegalPositionError struct{}

func (e *IllegalPositionError) Error() string {
	return "Illegal position"
}

func CanQueenAttack(white string, black string) (attack bool, err error) {
	w, whiteError := chess.ParsePosition(white)

	if whiteError != nil {
		return false, whiteError
	}

	b, blackError := chess.ParsePosition(black)

	if blackError != nil {
		return false, blackError
	}

	isSameRank := w.Rank == b.Rank
	isSameFile := w.File == b.File

	isSamePosition := isSameFile && isSameRank

	if isSamePosition {
		return false, &IllegalPositionError{}
	}

	isSameDiagonal := Abs(w.Rank, b.Rank) == Abs(w.File, b.File)

	return isSameRank || isSameFile || isSameDiagonal, nil
}
