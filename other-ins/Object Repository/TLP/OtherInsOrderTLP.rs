<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>OtherInsOrderTLP</name>
   <tag></tag>
   <elementGuidId>663bb604-bfc6-4783-97e9-d40e7221585e</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;UserID\&quot;: \&quot;${id}\&quot;,\n    \&quot;MaskapaiID\&quot;: \&quot;${maskapai_id}\&quot;,\n    \&quot;TanggalOrder\&quot;: \&quot;${orderdate}\&quot;,\n    \&quot;Page\&quot;: \&quot;1\&quot;,\n    \&quot;Source\&quot;: \&quot;${source}\&quot;\n}&quot;,
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
      <webElementGuid>0ffe5780-9448-4050-bb08-8b7f97ea1c2a</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${token}</value>
      <webElementGuid>99579e55-c07a-4912-aaf9-45e7c7b7e7da</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.3.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://api-dev.bcaf.id:8445/OtherInsOrderTLP</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>'UTK11Sejg49Y8q1YUFk4yqVUSWGB'</defaultValue>
      <description></description>
      <id>75e836b4-a49b-470d-82e5-1b529f243ad2</id>
      <masked>false</masked>
      <name>token</name>
   </variables>
   <variables>
      <defaultValue>'TLP'</defaultValue>
      <description></description>
      <id>873db74e-ab2e-4b7f-a896-8d4f1e428e7e</id>
      <masked>false</masked>
      <name>id</name>
   </variables>
   <variables>
      <defaultValue>'TLP'</defaultValue>
      <description></description>
      <id>fd75dfd9-16c7-496a-97ab-6f8d713bf95b</id>
      <masked>false</masked>
      <name>maskapai_id</name>
   </variables>
   <variables>
      <defaultValue>'AXA'</defaultValue>
      <description></description>
      <id>66da2905-e37c-44b5-a206-992ad55b2901</id>
      <masked>false</masked>
      <name>source</name>
   </variables>
   <variables>
      <defaultValue>'2022-04-25'</defaultValue>
      <description></description>
      <id>006b4914-e253-4484-beca-95a064b867d7</id>
      <masked>false</masked>
      <name>orderdate</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
