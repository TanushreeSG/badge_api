<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>UPDATE PATCH</name>
   <tag></tag>
   <elementGuidId>677551f3-8529-4002-84e9-f3a6c8b02aa0</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;name\&quot;: \&quot;morpheus\&quot;,\n    \&quot;job\&quot;: \&quot;zion resident\&quot;\n}&quot;,
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
      <webElementGuid>c8a15ff6-305c-4153-ac5c-02a8cfb7663a</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.3.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>PATCH</restRequestMethod>
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

import groovy.json.JsonSlurper

// Get the current response
ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

// Parse the response content
def jsonResponse = new JsonSlurper().parseText(response.getResponseText())

// Verify 'name' property in the response
WS.verifyElementPropertyValue(response, 'name', &quot;morpheus&quot;)

// Verify 'job' property in the response
WS.verifyElementPropertyValue(response, 'job', &quot;zion resident&quot;)

// Verify 'updatedAt' property in the response
assertThat(jsonResponse.updatedAt).isNotNull()

// Verify response status code
WS.verifyResponseStatusCode(response, 200)
assertThat(response.getStatusCode()).isEqualTo(200)

// Verify response status code within a range
assertThat(response.getStatusCode()).isIn(200, 201, 202)
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
