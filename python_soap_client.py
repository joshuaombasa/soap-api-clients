from zeep import Client

# Define WSDL URL
wsdl_url = "http://webservices.oorsprong.org/websamples.countryinfo/CountryInfoService.wso?WSDL"

# Create a SOAP client
client = Client(wsdl=wsdl_url)

# Call the function to get the capital city of Kenya
result = client.service.CapitalCity('KE')

# Print the result
print(f"The capital city of Kenya is: {result}")
