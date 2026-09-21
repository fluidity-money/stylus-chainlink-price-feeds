package stylus_chainlink_price_feeds

import (
	_ "embed"
)

//go:embed abi.json
var abiB []byte

var abi, _ = ethAbi.JSON(bytes.NewReader(abiB))

