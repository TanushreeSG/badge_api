<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>CREATE</name>
   <tag></tag>
   <elementGuidId>14381987-bd66-4092-8afc-29f13fc3025b</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;name\&quot;: \&quot;morpheus\&quot;\n    \&quot;job\&quot;: \&quot;zion resident\&quot;\n}\n&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>69ca22a5-1950-443b-b962-b3d5e24a9d8f</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.3.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>https://reqres.in/api/users/2</restUrl>
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
