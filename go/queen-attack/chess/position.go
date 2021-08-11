package chess

import (
	"fmt"
)

type Position struct {
	Rank byte
	File byte
}

type InvalidPositionFormatError struct {
	Text string
}

func (e *InvalidPositionFormatError) Error() string {
	return fmt.Sprintf("Invalid chess position format %s", e.Text)
}

type InvalidRankError struct {
	Rank byte
}

func (e *InvalidRankError) Error() string {
	return fmt.Sprintf("Invalid rank %d", e.Rank)
}

type InvalidFileError struct {
	File byte
}

func (e *InvalidFileError) Error() string {
	return fmt.Sprintf("Invalid file %d", e.File)
}

func ParsePosition(notation string) (Position, error) {
	var position Position

	if len(notation) != 2 {
		return position, &InvalidPositionFormatError{notation}
	}

	file, rank := notation[0], notation[1]

	fileOnBoard := file >= 'a' && file <= 'h'
	rankOnBoard := rank >= '1' && rank <= '8'

	switch {
	case !fileOnBoard:
		return position, &InvalidFileError{file}
	case !rankOnBoard:
		return position, &InvalidRankError{rank}
	default:
		return Position{rank, file}, nil
	}
}
