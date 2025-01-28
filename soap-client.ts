import * as soap from 'soap';

// Define WSDL URL
const wsdlUrl = "http://webservices.oorsprong.org/websamples.countryinfo/CountryInfoService.wso?WSDL";

// Create a SOAP client
soap.createClient(wsdlUrl, (err, client) => {
    if (err) {
        console.error("Error creating SOAP client:", err);
        return;
    }

    // Call the function to get the capital city of Kenya
    client.CapitalCity({ sCountryISOCode: 'KE' }, (err: any, result: any) => {
        if (err) {
            console.error("Error calling SOAP service:", err);
            return;
        }

        console.log(`The capital city of Kenya is: ${result?.CapitalCityResult}`);
    });
});
