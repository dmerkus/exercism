// Package gigasecond implements a function that adds a Gigasecond to a given Time
package gigasecond

import "time"

// AddGigasecond adds a Gigasecond to t
func AddGigasecond(t time.Time) time.Time {
	return t.Add(1e9 * time.Second)
}
