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
import org.openqa.selenium.Keys as Keys

WebUI.openBrowser('')

WebUI.navigateToUrl('https://demo.epuskesmas.id/')

WebUI.click(findTestObject('Object Repository/hapuspasien/Page_ePuskesmas/li_Infokes Manajemen Pasien                _bf0a20'))

WebUI.click(findTestObject('Object Repository/hapuspasien/Page_ePuskesmas/span_ePuskesmas'))

WebUI.setText(findTestObject('Object Repository/hapuspasien/Page_e-Puskesmas - Masuk/input_Indonesia_email'), 'darel')

WebUI.setEncryptedText(findTestObject('Object Repository/hapuspasien/Page_e-Puskesmas - Masuk/input_Indonesia_password'), 
    'iSJZYhOg0uc=')

WebUI.click(findTestObject('Object Repository/hapuspasien/Page_e-Puskesmas - Masuk/button_Login'))

WebUI.click(findTestObject('Object Repository/hapuspasien/Page_e-Puskesmas -/button_PUSKESMAS PUSKESMAS1'))

WebUI.click(findTestObject('Object Repository/hapuspasien/Page_e-Puskesmas -/a_Pendaftaran'))

WebUI.click(findTestObject('Object Repository/hapuspasien/Page_e-Puskesmas -/a_Pasien  KK'))

WebUI.click(findTestObject('Object Repository/hapuspasien/Page_e-Puskesmas - Pasien dan Kartu Keluarga/span_175'))

WebUI.click(findTestObject('Object Repository/hapuspasien/Page_e-Puskesmas - Pasien dan Kartu Keluarga/span_174'))

WebUI.click(findTestObject('Object Repository/hapuspasien/Page_e-Puskesmas - Pasien dan Kartu Keluarga/td_Label 2_text-center'))

WebUI.click(findTestObject('Object Repository/hapuspasien/Page_e-Puskesmas - Pasien dan Kartu Keluarga/input_Label 2_check'))

WebUI.click(findTestObject('Object Repository/hapuspasien/Page_e-Puskesmas - Pasien dan Kartu Keluarga/button_Hapus'))

WebUI.click(findTestObject('Object Repository/hapuspasien/Page_e-Puskesmas - Pasien dan Kartu Keluarga/a_Darel PUSKESMAS PUSKESMAS1'))

WebUI.click(findTestObject('Object Repository/hapuspasien/Page_e-Puskesmas - Pasien dan Kartu Keluarga/a_Keluar'))

