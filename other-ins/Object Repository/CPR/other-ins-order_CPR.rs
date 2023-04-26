<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>other-ins-order_CPR</name>
   <tag></tag>
   <elementGuidId>ddb47161-e947-44a9-acb3-f72eefe35089</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;OrderInsCarSmartWarrantyRq\&quot;: {\n        \&quot;UserId\&quot;: \&quot;${id}\&quot;,\n        \&quot;OrderDate\&quot;: \&quot;${order_date}\&quot;,\n        \&quot;Page\&quot;: \&quot;1\&quot;\n    },\n    \&quot;Source\&quot;: \&quot;${source}\&quot;\n}&quot;,
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
      <webElementGuid>79e700c5-3107-4f99-8182-34cd522e096c</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.3.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://api-dev.bcaf.id:8445/otherorderinscarwarranty</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>'ewNZQHrDyoayIexbE4sxzeujJQGJ'</defaultValue>
      <description></description>
      <id>98110c03-e725-4674-8b98-cc6536d12359</id>
      <masked>false</masked>
      <name>token</name>
   </variables>
   <variables>
      <defaultValue>'CPR 03'</defaultValue>
      <description></description>
      <id>c33e4e4a-893c-45c1-9af9-8b9d041bad5e</id>
      <masked>false</masked>
      <name>id</name>
   </variables>
   <variables>
      <defaultValue>'2021-12-31'</defaultValue>
      <description></description>
      <id>2ac27810-4a32-4466-827d-bd83c26eaf87</id>
      <masked>false</masked>
      <name>order_date</name>
   </variables>
   <variables>
      <defaultValue>'PGI'</defaultValue>
      <description></description>
      <id>8c74e358-aa2c-473d-abe2-1edc3a7317ce</id>
      <masked>false</masked>
      <name>source</name>
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
