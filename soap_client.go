package main

import (
	"fmt"
	"log"

	"github.com/hooklift/gowsdl/soap"
)

func main() {
	// Define WSDL URL
	wsdlURL := "http://webservices.oorsprong.org/websamples.countryinfo/CountryInfoService.wso?WSDL"

	// Create a SOAP client
	client := soap.NewClient(wsdlURL)

	// Define request parameters
	request := struct {
		SISOCode string `xml:"sCountryISOCode"`
	}{SISOCode: "KE"}

	// Define response structure
	response := struct {
		CapitalCityResult string `xml:"CapitalCityResult"`
	}{}

	// Call the SOAP method
	err := client.Call("http://webservices.oorsprong.org/websamples.countryinfo/CountryInfoService/CapitalCity", request, &response)
	if err != nil {
		log.Fatalf("SOAP request failed: %v", err)
	}

	// Print the result
	fmt.Printf("The capital city of Kenya is: %s\n", response.CapitalCityResult)
}
