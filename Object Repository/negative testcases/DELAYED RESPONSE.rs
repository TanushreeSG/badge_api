<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>DELAYED RESPONSE</name>
   <tag></tag>
   <elementGuidId>3c3942fc-be69-4256-bce0-ace3fb10ab3e</elementGuidId>
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
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://reqres.in/user?delay=3</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <verificationScript>//import static org.assertj.core.api.Assertions.*
//
//import com.kms.katalon.core.testobject.RequestObject
//import com.kms.katalon.core.testobject.ResponseObject
//import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
//import com.kms.katalon.core.webservice.verification.WSResponseManager
//
//import groovy.json.JsonSlurper
//import internal.GlobalVariable as GlobalVariable
//
//RequestObject request = WSResponseManager.getInstance().getCurrentRequest()
//
//ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()
//


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
