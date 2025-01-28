import * as soap from 'soap';

const wsdlUrl = "http://webservices.oorsprong.org/websamples.countryinfo/CountryInfoService.wso?WSDL";

async function getCapitalCity(countryCode: string) {
    try {
        const client = await soap.createClientAsync(wsdlUrl);
        const [result] = await client.CapitalCityAsync({ sCountryISOCode: countryCode });
        console.log(`The capital city of ${countryCode} is: ${result.CapitalCityResult}`);
    } catch (error) {
        console.error("Error:", error);
    }
}

getCapitalCity('KE');
