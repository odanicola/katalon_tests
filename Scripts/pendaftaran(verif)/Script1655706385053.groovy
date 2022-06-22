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

WebUI.click(findTestObject('Object Repository/pendaftaran(verif)/Page_ePuskesmas/li_Infokes Manajemen Pasien                _bf0a20'))

WebUI.click(findTestObject('Object Repository/pendaftaran(verif)/Page_ePuskesmas/span_ePuskesmas'))

WebUI.setText(findTestObject('Object Repository/pendaftaran(verif)/Page_e-Puskesmas - Masuk/input_Indonesia_email'), 'darel')

WebUI.setEncryptedText(findTestObject('Object Repository/pendaftaran(verif)/Page_e-Puskesmas - Masuk/input_Indonesia_password'), 
    'iSJZYhOg0uc=')

WebUI.click(findTestObject('Object Repository/pendaftaran(verif)/Page_e-Puskesmas - Masuk/button_Login'))

WebUI.click(findTestObject('Object Repository/pendaftaran(verif)/Page_e-Puskesmas -/button_PUSKESMAS PUSKESMAS1'))

WebUI.click(findTestObject('Object Repository/pendaftaran(verif)/Page_e-Puskesmas -/a_Pendaftaran'))

WebUI.click(findTestObject('Object Repository/pendaftaran(verif)/Page_e-Puskesmas -/a_Pasien  KK'))

WebUI.click(findTestObject('Object Repository/pendaftaran(verif)/Page_e-Puskesmas - Pasien dan Kartu Keluarga/a_Buat Baru'))

WebUI.setText(findTestObject('Object Repository/pendaftaran(verif)/Page_e-Puskesmas - Pasien - Buat Baru/input__MPasiennik'), 
    '0000000000001234')

WebUI.setText(findTestObject('Object Repository/pendaftaran(verif)/Page_e-Puskesmas - Pasien - Buat Baru/input__MPasiennama'), 
    'dimas')

WebUI.setText(findTestObject('Object Repository/pendaftaran(verif)/Page_e-Puskesmas - Pasien - Buat Baru/input__MPasientanggal_lahir'), 
    '09-02-1999')

WebUI.setText(findTestObject('Object Repository/pendaftaran(verif)/Page_e-Puskesmas - Pasien - Buat Baru/input__MPasientempat_lahir'), 
    'siantar')

WebUI.setText(findTestObject('Object Repository/pendaftaran(verif)/Page_e-Puskesmas - Pasien - Buat Baru/input__MPasienno_hp'), 
    '081355882298')

WebUI.setText(findTestObject('Object Repository/pendaftaran(verif)/Page_e-Puskesmas - Pasien - Buat Baru/input__propinsi_nama'), 
    'sumatera utara')

WebUI.click(findTestObject('Object Repository/pendaftaran(verif)/Page_e-Puskesmas - Pasien - Buat Baru/div_SUMATERA UTARA'))

WebUI.setText(findTestObject('Object Repository/pendaftaran(verif)/Page_e-Puskesmas - Pasien - Buat Baru/input__propinsi_nama'), 
    'SUMATERA UTARA')

WebUI.setText(findTestObject('Object Repository/pendaftaran(verif)/Page_e-Puskesmas - Pasien - Buat Baru/input__kota_nama'), 
    'kabupaten karo')

WebUI.click(findTestObject('Object Repository/pendaftaran(verif)/Page_e-Puskesmas - Pasien - Buat Baru/input__kota_nama'))

WebUI.click(findTestObject('Object Repository/pendaftaran(verif)/Page_e-Puskesmas - Pasien - Buat Baru/div_KABUPATEN KARO SUMATERA UTARA'))

WebUI.setText(findTestObject('Object Repository/pendaftaran(verif)/Page_e-Puskesmas - Pasien - Buat Baru/input__kota_nama'), 
    'KABUPATEN KARO')

WebUI.setText(findTestObject('Object Repository/pendaftaran(verif)/Page_e-Puskesmas - Pasien - Buat Baru/input__kecamatan_nama'), 
    'berastagi')

WebUI.click(findTestObject('Object Repository/pendaftaran(verif)/Page_e-Puskesmas - Pasien - Buat Baru/input__kecamatan_nama'))

WebUI.click(findTestObject('Object Repository/pendaftaran(verif)/Page_e-Puskesmas - Pasien - Buat Baru/div_BERASTAGI KABUPATEN KARO'))

WebUI.setText(findTestObject('Object Repository/pendaftaran(verif)/Page_e-Puskesmas - Pasien - Buat Baru/input__kecamatan_nama'), 
    'BERASTAGI')

WebUI.click(findTestObject('Object Repository/pendaftaran(verif)/Page_e-Puskesmas - Pasien - Buat Baru/input__kelurahan_nama'), 
    FailureHandling.STOP_ON_FAILURE)

WebUI.setText(findTestObject('Object Repository/pendaftaran(verif)/Page_e-Puskesmas - Pasien - Buat Baru/input__kelurahan_nama'), 
    'gurusinga')

WebUI.click(findTestObject('Object Repository/pendaftaran(verif)/Page_e-Puskesmas - Pasien - Buat Baru/div_GURUSINGA BERASTAGI'))

WebUI.setText(findTestObject('Object Repository/pendaftaran(verif)/Page_e-Puskesmas - Pasien - Buat Baru/textarea__MPasienalamat'), 
    'jl udara')

WebUI.setText(findTestObject('Object Repository/pendaftaran(verif)/Page_e-Puskesmas - Pasien - Buat Baru/input__MPasienrt'), 
    '001')

WebUI.setText(findTestObject('Object Repository/pendaftaran(verif)/Page_e-Puskesmas - Pasien - Buat Baru/input__MPasienrw'), 
    '002')

WebUI.click(findTestObject('Object Repository/pendaftaran(verif)/Page_e-Puskesmas - Pasien - Buat Baru/input__pekerjaan_nama'))

WebUI.setText(findTestObject('Object Repository/pendaftaran(verif)/Page_e-Puskesmas - Pasien - Buat Baru/input__pekerjaan_nama'), 
    'guru')

WebUI.click(findTestObject('Object Repository/pendaftaran(verif)/Page_e-Puskesmas - Pasien - Buat Baru/div_GURU'))

WebUI.selectOptionByValue(findTestObject('Object Repository/pendaftaran(verif)/Page_e-Puskesmas - Pasien - Buat Baru/select_- Pilih -                           _05c17e'), 
    'HINDU', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/pendaftaran(verif)/Page_e-Puskesmas - Pasien - Buat Baru/select_- Pilih -                           _ba31d3'), 
    'DIPLOMA IV/STRATA I', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/pendaftaran(verif)/Page_e-Puskesmas - Pasien - Buat Baru/select_- Pilih -                           _7ac6b2'), 
    'BELUM KAWIN', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/pendaftaran(verif)/Page_e-Puskesmas - Pasien - Buat Baru/select_- Pilih -                           _9caa3e'), 
    'KEPALA KELUARGA', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/pendaftaran(verif)/Page_e-Puskesmas - Pasien - Buat Baru/select_- Pilih -                           _73a496'), 
    'INDONESIA', true)

WebUI.click(findTestObject('Object Repository/pendaftaran(verif)/Page_e-Puskesmas - Pasien - Buat Baru/button_Simpan'))

WebUI.click(findTestObject('Object Repository/pendaftaran(verif)/Page_e-Puskesmas - Pasien - Lihat Data/a_Lihat Semua'))

WebUI.click(findTestObject('Object Repository/pendaftaran(verif)/Page_e-Puskesmas - Pasien dan Kartu Keluarga/a_Darel PUSKESMAS PUSKESMAS1'))

WebUI.click(findTestObject('Object Repository/pendaftaran(verif)/Page_e-Puskesmas - Pasien dan Kartu Keluarga/a_Keluar'))

WebUI.closeBrowser()

