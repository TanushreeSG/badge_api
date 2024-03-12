<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>DELAYED RESPONSE</name>
   <tag></tag>
   <elementGuidId>009cfb10-a6eb-433f-a3ba-9508b45a4d9e</elementGuidId>
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
   <restUrl>https://reqres.in/api/users?delay=3</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()


WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)


assertThat(response.getStatusCode()).isIn(Arrays.asList(200, 201, 202))
WS.verifyElementPropertyValue(response, 'page', 1)
WS.verifyElementPropertyValue(response, 'per_page', 6)
WS.verifyElementPropertyValue(response, 'total', 12)
WS.verifyElementPropertyValue(response, 'total_pages', 2)
WS.verifyElementPropertyValue(response, 'data[0].id', 1)
WS.verifyElementPropertyValue(response, 'data[0].email', &quot;george.bluth@reqres.in&quot;)
WS.verifyElementPropertyValue(response, 'data[0].first_name', &quot;George&quot;)
WS.verifyElementPropertyValue(response, 'data[0].last_name', &quot;Bluth&quot;)
WS.verifyElementPropertyValue(response, 'data[0].avatar', &quot;https://reqres.in/img/faces/1-image.jpg&quot;)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
