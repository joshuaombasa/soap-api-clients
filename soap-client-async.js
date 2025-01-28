const soap = require('soap');

const wsdlUrl = "http://webservices.oorsprong.org/websamples.countryinfo/CountryInfoService.wso?WSDL";

async function getCapitalCity(countryCode) {
    try {
        const client = await soap.createClientAsync(wsdlUrl);
        const [result] = await client.CapitalCityAsync({ sCountryISOCode: countryCode });
        console.log(`The capital city of ${countryCode} is: ${result.CapitalCityResult}`);
    } catch (error) {
        console.error("Error:", error);
    }
}

getCapitalCity('KE');
