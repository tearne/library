package main

import (
	"encoding/json"
	"fmt"
	"github.com/mitchellh/mapstructure"
)

type Outer struct {
	Meta map[string]interface{} `json:"meta"`
	Particle Particle `json:"particle"`
}

type Particle struct {
	S []float64 `json:"s"`
	W float64 `json:"w"`
	P Parameter `json:"p"`
}

type Parameter struct {
	alpha float64 `json:"alpha"`
	beta  float64 `json:"beta"`
}

func main() {

	json_str := `
		{
			"meta": {},
			"particle" : {
				"s" : [10,20,30],
				"w" : 0.12345,
				"p" : {
					"alpha": 0.1,
					"beta": 10.1
				}
			}
		}
	`

	//https://blog.golang.org/json

	// Deserialising everything
	var outer Outer
	err := json.Unmarshal([]byte(json_str), &outer)
	if err != nil {
		panic(err)
	}
	fmt.Printf("Whole document: %+v\n", outer)
	// > Whole document: {Meta:map[] Particle:{S:[10 20 30] W:0.12345 P:{alpha:0 beta:0}}}

	// OR

	// Deserialising just the 'particle' bit
	var temp interface{}
	err = json.Unmarshal([]byte(json_str), &temp)
	if err != nil {
		panic(err)
	}
	var particle Particle
	cfg := &mapstructure.DecoderConfig{
        Metadata: nil,
        Result:   &particle,
        TagName:  "json",
    }
	decoder, _ := mapstructure.NewDecoder(cfg)
	sub_section := temp.(map[string]interface{})["particle"]
	decoder.Decode(sub_section)
	fmt.Printf("Subesection: %+v\n", particle)
	// > Subesection: {S:[10 20 30] W:0.12345 P:{alpha:0 beta:0}}
}
