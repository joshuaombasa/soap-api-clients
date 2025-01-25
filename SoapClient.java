import org.oorsprong.www.websamples.countryinfo.CountryInfoService;
import org.oorsprong.www.websamples.countryinfo.CountryInfoServiceSoap;

public class SoapClient {
    public static void main(String[] args) {
        // Instantiate the SOAP service
        CountryInfoService service = new CountryInfoService();
        CountryInfoServiceSoap port = service.getCountryInfoServiceSoap();

        // Call the SOAP method to get the capital city for Kenya
        String capitalCity = port.capitalCity("KE");

        // Print the result
        System.out.println("The capital city of Kenya is: " + capitalCity);
    }
}
