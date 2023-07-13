import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys
import org.testng.Assert as Keys
import org.testng.asserts.SoftAssert

   response = WS.sendRequestAndVerify(findTestObject('Policy_Info', [('ComCode') : 'SB', ('PolicyNumber') : 'ULP001693']))

   WS.verifyResponseStatusCode(response, 200)
   
   WS.verifyElementPropertyValue(response, 'Owners[0].FirstName', 'Samyuktha')
   
   def slurper = new groovy.json.JsonSlurper()
   def result = slurper.parseText(response.getResponseBodyContent())
   
   def PolicyNumber = result.PolicyNumber
   print("PolicyNumber :"+ PolicyNumber)
   
   def TaxCategory = result.TaxCategory
   print("TaxCategory :"+ TaxCategory)
   
   def CashValue = result.CashValue
   print("CashValue :"+ CashValue)
   
   def FaceAmount = result.FaceAmount
   print("FaceAmount :"+ FaceAmount)
   
   def IssueDate = result.IssueDate
   print("IssueDate :"+ IssueDate)
   
   def EffectiveDate = result.EffectiveDate
   print("EffectiveDate :"+ EffectiveDate)
   
   def ApplicationDate = result.ApplicationDate
   print("ApplicationDate :"+ ApplicationDate)
   
   def MecStatus = result.MecStatus
   print("MecStatus :"+ MecStatus)
   
   def MecStatusDesc = result.MecStatusDesc
   print("MecStatusDesc :"+ MecStatusDesc)
   
   def MaturityExpiryDate = result.MaturityExpiryDate
   print("MaturityExpiryDate :"+ MaturityExpiryDate)
   
   def RenewalDate = result.RenewalDate
   print("RenewalDate :"+ RenewalDate)
   
   def Status = result.Status
   print("Status :"+ Status)
   
   def StatusDesc = result.StatusDesc
   print("StatusDesc :"+ StatusDesc)
   
   def IssueState = result.IssueState
   print("IssueState :"+ IssueState)
   
   def PlanDescription = result.PlanDescription
   print("PlanDescription :"+ PlanDescription)
   
   def ProductCode = result.ProductCode
   print("ProductCode :"+ ProductCode)
   
   def ProductLine = result.ProductLine
   print("ProductLine :"+ ProductLine)
   
   def LengthOfTerm = result.LengthOfTerm
   print("LengthOfTerm :"+ LengthOfTerm)
   
   def VitalityStatus = result.VitalityStatus
   print("VitalityStatus :"+ VitalityStatus)
   
   def LastUpdated = result.LastUpdated
   print("LastUpdated :"+ LastUpdated)
   
   def PersonType = result.Owners[0].PersonType
   print("PersonType :"+ PersonType)
   
   def PersonTypeDesc = result.Owners[0].PersonTypeDesc
   print("PersonTypeDesc :"+ PersonTypeDesc)
   
   def FullName = result.Owners[0].FullName
   print("FullName :"+ FullName)
   
   def FirstName = result.Owners[0].FirstName
   print("LastUpdated :"+ LastUpdated)
   
   def MiddleName = result.Owners[0].MiddleName
   print("MiddleName :"+ MiddleName)
   
   def LastName = result.Owners[0].LastName
   print("LastName :"+ LastName)
   
   def Suffix = result.Owners[0].Suffix
   print("Suffix :"+ Suffix)
   
   def OrgName = result.Owners[0].OrgName
   print("OrgName :"+ OrgName)
   
   def PrefComm = result.Owners[0].PrefComm
   print("PrefComm :"+ PrefComm)
   
   def CustomerID = result.Owners[0].CustomerID
   print("CustomerID :"+ CustomerID)
   
   def TokenNumber = result.TokenNumber
   print("TokenNumber :"+ TokenNumber)
   
   def AccountValue = result.AccountValue
   print("AccountValue :"+ AccountValue)
   
   def AccountSurrenderValue = result.AccountSurrenderValue
   print("AccountSurrenderValue :"+ AccountSurrenderValue)
   
   def PackageAcknowledgement = result.PackageAcknowledgement
   print("PackageAcknowledgement :"+ PackageAcknowledgement)
   
   def PackageAcknowledgementDate = result.PackageAcknowledgementDate
   print("PackageAcknowledgementDate :"+ PackageAcknowledgementDate)
   
   def MatchLifetimeLimit = result.MatchLifetimeLimit
   print("MatchLifetimeLimit :"+ MatchLifetimeLimit)
   
   def FormNumber = result.FormNumber
   print("FormNumber :"+ FormNumber)
   
   def PlanCode = result.PlanCode
   print("PlanCode :"+ PlanCode)
   
   def FreeLook = result.FreeLook
   print("FreeLook :"+ FreeLook)
   