package letter

import "sync"

// FreqMap records the frequency of each rune in a given text.
type FreqMap map[rune]int

// Frequency counts the frequency of each rune in a given text and returns this
// data as a FreqMap.
func Frequency(s string) FreqMap {
	m := FreqMap{}
	for _, r := range s {
		m[r]++
	}
	return m
}

func ConcurrentFrequency(texts []string) FreqMap {
	wg := sync.WaitGroup{}

	runes := FreqMap{}
	rw := sync.RWMutex{}

	for _, text := range texts {
		wg.Add(1)
		go func(text string, wg *sync.WaitGroup) {
			defer wg.Done()

			for textRune, frequency := range Frequency(text) {
				rw.Lock()
				runes[textRune] += frequency
				rw.Unlock()
			}
		}(text, &wg)
	}

	wg.Wait()

	return runes
}
