package raindrops_test

import (
	"testing"

	"github.com/hsmtkk/psychic-dollop/raindrops"
	"github.com/stretchr/testify/assert"
)

func TestRainDrops(t *testing.T) {
	assert.Equal(t, "Plong", raindrops.RainDrops(28))
	assert.Equal(t, "PlingPlang", raindrops.RainDrops(30))
	assert.Equal(t, "34", raindrops.RainDrops(34))
}
