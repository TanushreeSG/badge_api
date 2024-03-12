<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>LIST USERS</name>
   <tag></tag>
   <elementGuidId>99d9e41b-762f-4998-8960-4b41a8130ff1</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <katalonVersion>9.3.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>https://reqres.in/users?page=2</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <verificationScript>

import static org.assertj.core.api.Assertions.*
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

if (response.getStatusCode() != 200) {
	println(&quot;Request failed with status code: ${response.getStatusCode()}&quot;)
	println(&quot;Response content: ${response.getResponseText()}&quot;)
	// Handle error condition here
} else {
	try {
		WS.verifyElementPropertyValue(response, 'status', 400)
		// Add more verification logic here if needed
	} catch (Exception e) {
		println(&quot;Verification failed:&quot;)
		println(e.getMessage())
		println(&quot;Response content: ${response.getResponseText()}&quot;)
		// Handle verification failure here
	}
}

</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
