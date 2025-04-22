<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Get_WeatherForecast</name>
   <tag></tag>
   <elementGuidId>4fc9a73f-74bc-40c9-8fc4-73c6c0da00d6</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${GlobalVariable.baseUrl}/data/2.5/forecast?lat=${GlobalVariable.lat}&amp;lon=${GlobalVariable.lon}&amp;cnt=${day}&amp;units=${units}&amp;appid=${GlobalVariable.apiKey}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.successCode</defaultValue>
      <description></description>
      <id>f1c9042c-8dca-4d38-ba9e-77542f9e8d20</id>
      <masked>false</masked>
      <name>expectedStatusCode</name>
   </variables>
   <variables>
      <defaultValue>'metric'</defaultValue>
      <description>Satuan suhu (metric untuk Celsius)</description>
      <id>2759af47-426d-4794-b11d-5e9b4c7f42f9</id>
      <masked>false</masked>
      <name>units</name>
   </variables>
   <variables>
      <defaultValue>5</defaultValue>
      <description>5 hari</description>
      <id>096f619e-cf98-4c6a-b5b0-be9cd5174b7b</id>
      <masked>false</masked>
      <name>day</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()
ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()
assert response.getStatusCode() == 200</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
