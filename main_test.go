package main

import (
	"testing"
)

func TestGetDigest(t *testing.T) {
	inputs := []string{"nishanth", "arun", "shiva"}
	outputs := []string{
		"2940514c4f5205eaf91133024970c928",
		"9ff37aeab33610230bf15b48ba26424b",
		"2dd3db25a5ff334703c19a2732939027",
	}

	for i, input := range inputs {
		digest := GetDigest(input)

		if digest != outputs[i] {
			t.Fatalf("want: %s, got: %s", outputs[i], digest)
		}
	}
}
