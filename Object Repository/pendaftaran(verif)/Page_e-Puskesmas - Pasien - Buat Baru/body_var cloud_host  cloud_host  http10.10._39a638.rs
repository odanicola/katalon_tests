<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>body_var cloud_host  cloud_host  http10.10._39a638</name>
   <tag></tag>
   <elementGuidId>10537fbe-8dc6-42da-80d1-9b0e2b72a0ee</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>//body</value>
      </entry>
      <entry>
         <key>CSS</key>
         <value>body</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>body</value>
      <webElementGuid>fcde8bfa-578d-4926-9fb6-291e375d1e5e</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
                            
	var cloud_host = &quot;&quot;;
									cloud_host = 'http://10.10.10.10/home';
				localStorage.setItem(&quot;epuskesmas-cloud-server&quot;, cloud_host);

	
	    #connection {
	        position: fixed;
	        font-weight: bold;
	        bottom: 5px;
	        left: 5px;
	        z-index: 99;
	        font-size: 16px;
	        border: none;
	        outline: none;
	        color: white;
	        cursor: pointer;
	        padding: 3px;
	        border-radius: 4px;
	    }
	    .online {
	      background: green;
	    }
	    .offline {
	      background: red;
	    }
	
	
	
	    window.onbeforeunload = function(event){
	    	var connection = document.getElementById('connection').innerHTML;
	    	localStorage.setItem('epuskesmas-connection-status', connection);
	    }
	    if ('serviceWorker' in navigator) {
	    	document.onreadystatechange = function(e){
	    		if(localStorage.getItem('epuskesmas-connection-status') != null &amp;&amp; localStorage.getItem('epuskesmas-connection-status').length > 0){
		            document.getElementById('connection').innerHTML = localStorage.getItem('epuskesmas-connection-status');
		            document.getElementById('connection').className = localStorage.getItem('epuskesmas-connection-status').toLowerCase();
	    		}
			}

	        window.addEventListener('load', function () {
	            navigator.serviceWorker.register('/service-worker.js').then(function(worker){
		    		if(localStorage.getItem('epuskesmas-connection-status') != null &amp;&amp; localStorage.getItem('epuskesmas-connection-status').length > 0){
		    			var loadConnection = localStorage.getItem('epuskesmas-connection-status').toLowerCase();
									                if(nextConnection == 'online'){
			            		sweetAlert('online');
			                }else{
			            		sweetAlert('offline', 'warning');
			                }
								    			sweetAlert(localStorage.getItem('epuskesmas-connection-status').toLowerCase());
		    		}
	            });
	        });
	        
	        function send_message_to_service_worker(messages){
	            return new Promise(function(resolve, reject){
	                var messageChannel = new MessageChannel();
	                messageChannel.port1.onmessage = function(event){
	                    if(event.data.error){
	                        reject(event.data.error);
	                    }else{
	                        resolve(event.data);
	                    }
	                };
	                navigator.serviceWorker.controller.postMessage(messages, [messageChannel.port2]);
	            });
	        }

			navigator.serviceWorker.getRegistrations().then(registrations => {
				registrations.forEach(registration => {
					registration.unregister()
				})
			})

	        navigator.serviceWorker.addEventListener('message', function(event){
	        	if(event.data.type == 'check connection'){
		            var previousConnection = document.getElementById('connection').className;
		            if(event.data.status != &quot;&quot;){
			            document.getElementById('connection').innerHTML = event.data.status;
			            document.getElementById('connection').className = event.data.status.toLowerCase();
		            }
		            var nextConnection = document.getElementById('connection').className;
								            if(previousConnection == 'offline'){
			                if(nextConnection == 'online'){
			            		sweetAlert('online');
			                }else{
			            		sweetAlert('offline', 'warning');
			                }
			            }else{
			                if(previousConnection == 'online'){
			                	if(nextConnection == 'offline'){
				            		sweetAlert('offline');
			                	}
			                }
			            }
						        	}
	        });

	        function sweetAlert(message = null, timer = 3000){
						        	switch(message) {
					  	case 'offline':
		                	Swal.fire({
		                       	html: '&lt;h5>ePuskesmas NG mendeteksi adanya gangguan pada koneksi internet.&lt;br>Pastikan perangkat anda terhubung dengan koneksi internet.&lt;h5>',
								imageUrl: 'https://demo.epuskesmas.id/images/o2o/offline.png',
								imageWidth: 400,
								imageHeight: 175,
								allowOutsideClick: false,
								showConfirmButton: &quot;1&quot;,
		                       	confirmButtonColor: '#224d09',
		                       	confirmButtonText: 'Ya, Alihkan ke ePuskesmas NG Offline!'
		                   	}).then((result) => {
		                       	if (result.value) {
		                        	if(cloud_host == localStorage.getItem('epuskesmas-cloud-server')){
		                      			window.location = localStorage.getItem('epuskesmas-cloud-server');
		                        	}
		                       	}
		                   	})
					    	break;
		        		default:
			  				Swal.fire({
		                        html: '&lt;h5>ePuskesmas NG telah terhubung (online).&lt;h5>',
								imageUrl: 'https://demo.epuskesmas.id/images/o2o/online.png',
								imageWidth: 400,
								imageHeight: 175,
		                        timer: timer,
								showConfirmButton: false
		                    });
					}
					        }

	        setInterval(function(){
	            send_message_to_service_worker({'type' : 'check connection', 'url' :''});
	        }, 8000);
	    }
	
                
        
            
                
                
                    
                        Toggle navigation
                        
                    
                
                
                                                                   
                                                                                                
                                        Pendaftaran 
                                        
                                                                                        Pasien &amp; KK
                                                                                                                                    Pendaftaran Pasien
                                                                                                                                    Rekam Medis
                                                                                                                                    Antrean
                                                                                         
                                            Panggil Antrean
                                                                                    
                                    
                                
                                                                    
                                        Pelayanan 
                                        
                                                                                            Pelayanan
                                                                                                Medis
                                                                                                                                                Resep
                                                                                                                                                Obat Pasien
                                                                                                                                                Penimbangan Balita
                                                                                                                                                Diare Advokasi
                                                                                                                                                Program Puskesmas
                                                                                                                                        
                                                                                            Pelayanan Luar Gedung
                                                                                                    Skrining Anak Sekolah
                                                                                                                                                   Skrining Santri
                                                                                                                                                    Skrining PTM
                                                                                                                                                    Sinkronisasi Kunjungan Sehat BPJS
                                                                                                                                                    Sinkronisasi Data Sihepi
                                                                                            
                                                                                            Pemeriksaan
                                                                                                Laboratorium
                                                                                                                                            
                                                                                            Pasien Pulang
                                                                                                Rujukan External
                                                                                               
                                                                                                Pasien Meninggal
                                                                                                                                                Pembayaran
                                                                                                                                                Detail Transaksi
                                                                                            
                                                                                            Survey Kepuasan/Testimoni Pasien
                                                                                                Survey Kepuasan &amp; Testimoni Pasien
                                                                                                                                                Pengelolaan Survey Kepuasan &amp; Testimoni Pasien
                                                                                            
                                                                                            Lroa
                                                                                                Lroa
                                                                                                                                    
                                    
                                                            
                                                            
                                    Pengelolaan 
                                    
                                                                                    Gudang Farmasi
                                                                                        Pemesanan Obat
                                                                                                                                    Penerimaan Obat
                                                                                                                                    Stok Obat
                                                                                                                                    Stok Opname
                                                                                                                                    Distribusi Obat
                                                                                    
                                                                                    Data Master
                                                                                        Pegawai
                                                                                                                                    Pengguna
                                                                                                                                                                                Ruangan
                                                                                                                                    Ruangan Akses
                                                                                                                                    Ruangan Puskesmas
                                                                                                                                    Ruangan JAKESH
                                                                                                                                    Puskesmas
                                                                                                                                    Perlengkapan
                                                                                                                                    Sasaran Program
                                                                                                                                                                                            Sasaran Proyeksi
                                                                                                                                                                                    Jadwal Pelayanan
                                                                                                                                    Jadwal Shift
                                                                                                                                    Libur
                                                                                                                                    Data Tindakan
                                                                                                                                    Data Imunisasi
                                                                                                                                    Kamar &amp; Bed
                                                                                                                                    Sekolah
                                                                                                                                    Asuransi
                                                                                                                                    Laboratorium
                                                                                                                                    Paket Laboratorium
                                                                                                                                    Obat
                                                                                                                                    Konfigurasi Dashboard Antrian
                                                                                                                                    Konfigurasi Form
                                                                                                                                    Konfigurasi Laboratorium
                                                                                                                                    SMS Konfigurasi
                                                                                                                                    Konfigurasi Pembulatan
                                                                                                                                    Asupan Makanan
                                                                                                                                    Industri
                                                                                                                                    PBF
                                                                                                                                                                                Komponen Tarif
                                                                                                                                    Metode Pembayaran
                                                                                                                                                                                Diagnosa
                                                                                                                                    Template Print
                                                                                                                                                                      Data Dasar Puskesmas
                                        
                                                                                    BPJS
                                                                                        Pengaturan BPJS
                                                                                                                                    RS Provider BPJS
                                                                                                                                    Poli FKTL BPJS
                                                                                                                                    Dokter BPJS
                                                                                                                                    Tindakan BPJS
                                                                                                                                    Laboratorium BPJS
                                                                                                                                    Obat BPJS
                                                                                                                                    Ruangan BPJS
                                                                                                                                    Pengaturan Bridging BPJS
                                                                                                                                    Sinkron BPJS
                                                                                                                                    Sinkron BPJS Jejaring
                                                                                                                                
                                        

                                        
                                                                                    SISRUTE
                                                                                        Konfigurasi
                                                                                                                                    Tujuan Rujukan
                                                                                                                                    Kriteria Khusus
                                                                                       
                                                                                        SDM
                                                                                                                                    Ruang Perawatan
                                                                                                                                    Pelayanan
                                                                                                                                    Sarana
                                                                                                                                    Kelas Perawatan
                                                                                                                                    Alkes
                                                                                    
                                                                                    Master Wilayah
                                                                                        Wilayah Kerja
                                                                                                                                    Wilayah Kerja Posyandu
                                                                                    
                                                                                    SIPTM
                                                                                        Puskesmas
                                                                                                                                    Diagnosis
                                                                                                                                    Sinkron SIPTM
                                                                                    
                                        
                                                                                    Scan
                                                                                        Scan QRCode
                                                                                                                                                                        Antrian
                                                                                        Antrian
                                                                                                                                                                        Speaker
                                                                                        Speaker
                                                                                                                                                                
                                
                            
                                                            
                                    GIS 
                                    
                                                                                Pasien
                                                                                                                        Penyakit
                                                                            
                                
                                                                                        
                                    Laporan 
                                    
                                                                                    Grafik
                                            Grafik
                                                                                                                            Dashboard SIP
                                        
                                                                                    Monitoring
                                                                                                                                    Penggunaan Fitur
                                                                                    
                                                                                    Laporan Harian
                                                                                                                                    Pendapatan Tindakan
                                                                                                                                    Kunjungan Pasien
                                                                                                                                    Kunjungan Pasien BPJS
                                                                                                                                    Pelayanan Pasien
                                                                                                                                    Pemeriksaan Medis
                                                                                                                                    Pelayanan Resep
                                                                                                                                    Pengeluaran Obat Pasien
                                                                                                                                    Pengeluaran Alat Kesehatan
                                                                                                                                    Pelayanan Laboratorium
                                                                                                                                    Rujukan Internal
                                                                                                                                    Rujukan External
                                                                                                                                    
                                                                                                    Tindakan Dokter/Perawat
                                                                                            
                                                                                                                                    Pemeriksaan Laboratorium
                                                                                                                                    Kinerja Puskesmas
                                                                                                                                    Pelayanan Ruangan
                                                                                                                                    Penyakit
                                                                                                                                    Asuhan Keperawatan
                                                                                                                                    Kunjungan PTM
                                                                                                                                    PKPR
                                                                                                                                    Alat Kesehatan Terbanyak
                                                                                                                                    Tindakan Terbanyak
                                                                                                                                    Obat Kadaluarsa
                                                                                                                                    Laporan Pendapatan
                                                                                                                                    Laporan Pendapatan Pendaftaran 
                                             
                                                                                                                                                                                        Laporan Pendapatan Kasir
                                                                                                                                                                                    Laporan Transaksi Kasir
                                                                                                                                                                                Laporan Skrining COVID-19
                                                                                                                                    Laporan Pengeluaran Obat
                                                                                                                                    Laporan Stok Opname
                                                                                                                                        Laporan Jasa Tenaga Medis
                                                                                                                                        Laporan Hepatitis C
                                                                                                                                        Laporan Diare
                                                                                                                                        Laporan Hepatitis B
                                                                                                                                        Pendapatan Laboratorium
                                                                                    
                                                                                    Laporan Mingguan
                                                                                        Laporan PWS Penyakit
                                                                                    
                                                                                    Laporan Bulanan
                                                                                        Laporan
                                                                                                                                    Data Dasar
                                                                                                                                    Kirim Report
                                                                                                                                    Monitoring Laporan
                                                                                                                                    SP3 LB1
                                                                                                                                    SP3 LB2 (LPLPO)
                                                                                                                                    SP3 LB3
                                                                                                                                    SP3 LB4
                                                                                                                                    UKP-1. Pelayanan Puskesmas
                                                                                                                                    UKP-2. Kesakitan Umum
                                                                                                                                    UKP-3. Kesakitan Gigi dan Mulut
                                                                                                                                    UKP-4. Data Kesakitan Terbanyak
                                                                                                                                    UKP-5. Data Kematian di Puskesmas
                                                                                                                                    UKP-6. LPLPO
                                                                                                                                    UKME-1. Promosi Kesehatan
                                            UKME-2. Kesehatan Lingkungan
                                            UKME-3. Gizi, Kesehatan Ibu dan Kesehatan Anak
                                            UKME-4. Imunisasi
                                            UKME-5. Pengendalian Penyakit Menular
                                            UKME-6. Pengendalian Penyakit Tidak Menular
                                            UKME-7. Perawatan Kesehatan Masyarakat
                                            Penyakit Menular Potensi KLB
                                            Penyakit Potensi KLB Menurut Desa/Kel
                                            Promosi Kesehatan Data Kemitraan
                                            UKMP. Kesehatan Kerja
                                            Program
                                                                                                                                    Pemeriksaan IMS
                                                                                                                                    Laporan Kunjungan Ibu Hamil
                                                                                                                                    Kohort KIA
                                                                                                                                    PKPR
                                                                                                                                    Konseling HIV
                                                                                                                                    Rekapitulasi MTBS MTBM
                                                                                                                                    PTM
                                                                                                                                    Kirim Laporan
                                                                                        Laporan Tahunan
                                            Laporan Eliminasi 3E
                                                                                        Laporan Tahunan
                                            Laporan Tahunan LSD 1
                                                                                                                                    Laporan Tahunan LSD 2
                                                                                                                                    Laporan Tahunan LSD 3
                                                                                                                                                                                
                                                                            
                                
                            
                                                    
                                                               
                        
                            
	.broadcastBox::-webkit-scrollbar-track
    {
        -webkit-box-shadow: inset 0 0 6px rgba(0,0,0,0.3);
        background-color: #F5F5F5;
      border-radius: 5px
    }

    .broadcastBox::-webkit-scrollbar
    {
        width: 10px;
        background-color: #F5F5F5;
      border-radius: 5px
    }

    .broadcastBox::-webkit-scrollbar-thumb
    {
        background-color: black;
        border: 2px solid black;
      border-radius: 5px
    }

    .fasBroadcast{
      font-size: 25pt;
      padding-bottom: 10px;
      color: black;
      margin-right: 40px;
      margin-left: 40px;
    }

    .broadcastBox{
      width: 400px;
      height: 0px;
      border-radius: 10px;
      transition: 0.5s;
      position: absolute;
      overflow-y: scroll;
      padding: 0px;
      left: -360px;
      margin-top: 5px;
      background-color: #F4F4F4;
      -webkit-box-shadow: 10px 10px 23px 0px rgba(0,0,0,0.2);
      -moz-box-shadow: 10px 10px 23px 0px rgba(0,0,0,0.1);
      box-shadow: 10px 10px 23px 0px rgba(0,0,0,0.1);
    }

    .fasBroadcast:hover {
      color: #d63031;
    }


    .displayBroadcast{
      position: relative;
    }

    .contBroadcast{
      position: absolute;
      top: 0;
      width: 100%;
      height: 100%;
      background-color: #F4F4F4;
    }

    .contBroadcast:empty{
      display: none;
    }

    .stickBroadcast{
      text-align: center;  
      display: block;
      font-size: 50pt;
      padding-top: 70px;
      padding-left: 80px
    }

    .stickBroadcast:hover{
      color: black;
    }

    .centBroadcast{
      text-align: center;
      display: block;
    }

    .secBroadcast{
      padding: 25px 10px;
      background-color: #F4F4F4;
      transition: 0.5s;
    }

    .profContBroadcast{
      padding-left: 15px;
    }

    .profileBroadcast{
      -webkit-clip-path: circle(50% at 50% 50%);
      clip-path: circle(50% at 50% 50%);
      width: 75px;
      float: left;
    }

    .txtBroadcast{
      vertical-align: top;
      font-size: 1.25rem;
      padding: 5px 10px 0px 115px;
    }

    .subBroadcast{
      font-size: 1rem;
      color: grey;
    }

    .newBroadcast{
      border-style: none none solid none;
      border-color: red;
    }

    .secBroadcast:hover{
      background-color: #BFBFBF;
    }

    .headerBroadcast{
      padding: 5px;
    }

    .isiContentBroadcast{
      padding: 10px;
    }

    .judulBroadcast{
      padding-bottom: 10px;
    }

    .dotBroadcast {
      height: 9px;
      width: 9px;
      background-color: #9D2D15;
      border-radius: 50%;
      display: inline-block;
    }

    .containerNotifBroadcast img {
      transition: transform 0.25s ease;
      cursor: zoom-in;
    }

    .zoomCheckNotif:checked ~ label > img {
      transform: scale(2);
      cursor: zoom-out;
    }


  



    var openBroadcast = &quot;true&quot;
    let numClicks = 0;
    function showHideBroadcast() {
        if (openBroadcast == &quot;false&quot;) {
          $('.broadcastBox').removeAttr(&quot;style&quot;);
          openBroadcast = &quot;true&quot;;
          return false
        } 

        numClicks++;
        if (numClicks === 1) {
          singleClickTimer = setTimeout(() => {
            numClicks = 0;
          }, 400);
        } else if (numClicks === 2) {
          clearTimeout(singleClickTimer);
          numClicks = 0;
          return false
        }

        $.ajax({
            url: &quot;https://demo.epuskesmas.id/notifikasi/getform&quot;,
            method: 'GET',
            dataType: &quot;json&quot;,
            data: {
                _token: &quot;1Qzy4frvPTBhCAOEI2lL6VeUfUPoVgVDUjUncrzY&quot;
            },
            success: function(data) {
                $(&quot;#isiNotifBroadcast&quot;).html(data.data)
                setTimeout(function (){
                      if(openBroadcast == &quot;true&quot;) {
                          $('.broadcastBox').css({
                              height: &quot;60vh&quot;
                          });
                          openBroadcast = &quot;false&quot;;
                      }
                }, 100);
            },
            error: function (xhr, ajaxOptions, thrownError) {
                alert(&quot;Terjadi kesalahan sistem, silahkan hubungi team support kami!&quot;, &quot;danger&quot;);
                console.log(xhr.status);
                console.log(thrownError);
            }
        });

        
    }


    function setPilihanBroadcast() {
        var broadcastPilih = $('select[name=&quot;broadcastPilihan&quot;]').val()

        $.ajax({
            url: &quot;https://demo.epuskesmas.id/notifikasi/getform&quot;,
            method: 'GET',
            dataType: &quot;json&quot;,
            data: {
                pilihanBroadcast: broadcastPilih,
                _token: &quot;1Qzy4frvPTBhCAOEI2lL6VeUfUPoVgVDUjUncrzY&quot;
            },
            success: function(data) {
                $('#contBroadcast').html(&quot;&quot;)
                $('#contBroadcast').html('&lt;br/>&lt;br/>&lt;span class=&quot;loading&quot; style=&quot;background-color:#F4F4F4!important;padding-top: 30px;padding-left: 90%;&quot;>&amp;nbsp;&lt;/span>')
                setTimeout(function (){
                  $('#contBroadcast').html(data.data)
                }, 300);
            },
            error: function (xhr, ajaxOptions, thrownError) {
                alert(&quot;Terjadi kesalahan sistem, silahkan hubungi team support kami!&quot;, &quot;danger&quot;);
                console.log(xhr.status);
                console.log(thrownError);
            }
        }); 
    }

    function bacaSelengkapnyaBroadcast(ids) {
      $('.broadcastBox').removeAttr(&quot;style&quot;);
      openBroadcast = &quot;true&quot;;

      $.blockUI({
        css: {
            border: 'none',
            padding: '15px',
            backgroundColor: '#000',
            '-webkit-border-radius': '10px',
            '-moz-border-radius': '10px',
            opacity: .5,
            fontSize: '8px',
            color: '#fff'
            },
            message: '&lt;h3>Memuat....&lt;/h3>'
        });

      $.unblockUI()

      $.ajax({
          url: &quot;https://demo.epuskesmas.id/notifikasi/show&quot;,
          method: 'GET',
          dataType: &quot;json&quot;,
          data: {
              ids: ids,
              _token: &quot;1Qzy4frvPTBhCAOEI2lL6VeUfUPoVgVDUjUncrzY&quot;
          },
          success: function(data) {
              $(&quot;#modal_notif_broadcast_notifikasi&quot;).modal({ backdrop: 'static'});
              $(&quot;#modal_notif_broadcast_notifikasi .modal-dialog&quot;).addClass(&quot;modal-lg&quot;);
              $('#modal_notif_broadcast_notifikasi .modal-title').html(data.tipe);
              $('#modal_notif_broadcast_notifikasi .modal-form').html(data.data);
              console.log(data.ressRead)
              if (data.ressRead == false) {
                $(&quot;#dotBroadcast&quot;).addClass('hidden')
              }
          },
          error: function (xhr, ajaxOptions, thrownError) {
              alert(&quot;Terjadi kesalahan sistem, silahkan hubungi team support kami!&quot;, &quot;danger&quot;);
              console.log(xhr.status);
              console.log(thrownError);
          }
      });   
    }

    function skemaAction() {
      let skema = $(&quot;#skema&quot;).val()
      $(&quot;#skemaAction&quot;).addClass('loading')
      var broadcastPilih = $('select[name=&quot;broadcastPilihan&quot;]').val()
      $.ajax({
          url: &quot;https://demo.epuskesmas.id/notifikasi/getformother&quot;,
          method: 'GET',
          dataType: &quot;json&quot;,
          data: {
              skema: skema,
              pilihanBroadcast: broadcastPilih,
              _token: &quot;1Qzy4frvPTBhCAOEI2lL6VeUfUPoVgVDUjUncrzY&quot;
          },
          success: function(data) {
            setTimeout(function (){
            $(&quot;#skemaAction&quot;).removeClass('loading')
            let totalSkema = Number(skema) + 1
              $(&quot;#skema&quot;).val(totalSkema)
              $(&quot;#isiDetailNotif&quot;).append(data.data)

              if(data.nextAction != &quot;true&quot;) {
                $(&quot;#skemaAction&quot;).addClass(&quot;hidden&quot;)
              }
            }, 300);
          },
          error: function (xhr, ajaxOptions, thrownError) {
              $(&quot;#skemaAction&quot;).removeClass('loading')
              alert(&quot;Terjadi kesalahan sistem, silahkan hubungi team support kami!&quot;, &quot;danger&quot;);
              console.log(xhr.status);
              console.log(thrownError);
          }
      }); 
    }
                        
                        
                                                             
                                                                        
                                         Belum Bridging
                                        8      
                                    
                                                                                                                        
                                                    
                                
                                    Darel PUSKESMAS PUSKESMAS1 
                                
                                
                                    Profil
                                                                            Update
                                                                        Umpan Balik
                                                                            Bantuan
                                                                        
                                    
                                        
                                            Keluar
                                        

                                        
                                            
                                        
                                    
                                
                            
                                            
                
            
        

        
        
        
        

        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        

        
                    backgroundChart = &quot;images/gis/bg-grafik.png&quot;;
                
        
        

        
        
            

    
        Buat Baru Pasien
        
                                            Simpan
                Pendaftaran
                        
            Lihat Semua
            Reset
        

    
    
        
        
        
            
                              
                    Data Pasien
                    
                                            
                            
                            Diverifikasi (lengkap)
                        
                    
                
                                              
                    Data PKM Lain
                    
                        
                        
                    
                
                                
                    No. eRM *
                    
                        
                                                            - Otomatis -
                                
                                                    
                    
                
                                
                    Penjamin
                    
                        
                                                                                                Umum

                                                                    BPJS Kesehatan

                                                                    Pemerintah Daerah Kota

                                                                    Bpjs ketenagakerjaan

                                                                    BPJS KESEHATAN RUJUK INTERNAL

                                                                    ASURANSI U/ HIDUP

                                                                    Umum Baru

                                                                    asuransi

                                                                    UMUM

                                                                                    
                    
                
                
                    No. Penjamin 
                    
                        
                            
                             
                        

                                                
                            
                        
                    
                
                
                    No. KK 
                    
                        
                    
                
                                
                    NIK  *
                    
                        
                            
                                                    
                    
                
                
                    
                  
                                      
                
                
                
                    Nama *
                    
                        
                    
                
                
                    Jenis Kelamin *
                    
                        
                            
                                
                                Laki-laki
                            
                            
                                
                                Perempuan
                            
                        
                    
                
                
                    Tanggal Lahir  *
                    
                        
                            
                            
                        
                    
                
                
                    Umur 
                    
                        
                            
                            Thn
                            
                            Bln
                            
                            Hr
                        
                    
                
                
                    Tempat Lahir  *
                    
                        
                    
                
                                            

            
                                
                    No. Dokumen RM 
                    
                        
                    
                
                                                
                    No. RM Lama 
                    
                        
                    
                
                                                
                    Golongan Darah 
                    
                        
                            - Pilih -
                                                                                                A
                                                                    B
                                                                    AB
                                                                    O
                                                                    A-
                                                                    A+
                                                                    B-
                                                                    B+
                                                                    AB+
                                                                    AB-
                                                                    O+
                                                                    O-
                                                                                    
                    
                
                
                    E-mail
                    
                        
                    
                
                
                    No. HP  *
                    
                        
                    
                
                                                
                    
                        Alamat Tempat Tinggal / KTP sama 
                    
                    
                                        
                            Propinsi  *
                            
                                
                                
                            
                        
                        
                            Kota/Kab  *
                            
                                
                                
                            
                        
                        
                            Kecamatan  *
                            
                                
                                
                            
                        
                        
                            Kelurahan/Desa  *
                            
                                
                                
                            
                        
                        
                            Dusun 
                            
                                
                                
                            
                        
                        
                            Alamat  *
                            
                                
                            
                        
                        
                            RT/RW   *
                            
                                
                                    
                                    
                                
                            
                        
                                        
                
                                        
            

            
                                
                    
                        
                            
                            Alamat KTP Jika Berbeda
                    
                    
                        
                            
                                Propinsi 
                                
                                    
                                    
                                
                            
                            
                                Kota/Kab 
                                
                                    
                                    
                                
                            
                            
                                Kecamatan 
                                
                                    
                                    
                                
                            
                            
                                Kelurahan/Desa 
                                
                                    
                                    
                                
                            
                            
                                Dusun 
                                
                                    
                                    
                                
                            
                            
                                Alamat 
                                
                                    
                                
                            
                            
                                RT/RW 
                                
                                    
                                        
                                        
                                    
                                
                            
                        

                    
                
                                
                    Pekerjaan  *
                    
                        
                        
                    
                
                                
                    Pekerjaan Suami
                    
                        
                        
                    
                
                
                    Instansi 
                    
                        
                    
                
                                
                    Agama  *
                    
                        
                            - Pilih -
                                                                                                ISLAM
                                                                    KATOLIK
                                                                    KRISTEN
                                                                    HINDU
                                                                    BUDDHA
                                                                    KONGHUCU
                                                                    LAINNYA
                                                                                    
                    
                
                
                    Pendidikan  *
                    
                        
                            - Pilih -
                                                                                                TIDAK/BELUM SEKOLAH
                                                                    BELUM TAMAT SD/SEDERAJAT
                                                                    TAMAT SD/SEDERAJAT
                                                                    SLTP/SEDERAJAT
                                                                    SLTA/SEDERAJAT
                                                                    DIPLOMA I
                                                                    DIPLOMA II
                                                                    AKADEMI/DIPLOMA III/SARJANA MUDA
                                                                    DIPLOMA IV/STRATA I
                                                                    STRATA II
                                                                    STRATA III
                                                                                    
                    
                
                
                    Status Perkawinan  *
                    
                        
                            - Pilih -
                                                                                                BELUM KAWIN
                                                                    KAWIN
                                                                    CERAI HIDUP
                                                                    CERAI MATI
                                                                                    
                    
                
                                
                    Tanggal Menikah
                    
                        
                            
                        
                        
                    
                
                
                    Status Keluarga  *
                    
                        
                            - Pilih -
                                                                                                KEPALA KELUARGA
                                                                    SUAMI
                                                                    ISTRI
                                                                    ANAK
                                                                    MENANTU
                                                                    CUCU
                                                                    ORANG TUA
                                                                    MERTUA
                                                                    PEMBANTU
                                                                    LAINNYA
                                                                                    
                    
                
                                                                
                    Warga Negara  *
                    
                        
                            - Pilih -
                                                                                                INDONESIA
                                                                    ASING
                                                                                    
                    
                
                
                    No. Paspor 
                    
                        
                            
                            
                        
                    
                
                
                    Nama Ayah/KK 
                    
                        
                    
                
                
                    Nama Ibu 
                    
                        
                    
                
                            

        
        
                            Simpan
                 Cetak
                 Kartu Pasien
                        
            
            
            Reset
        
    



history.pushState(null, null, location.href);
window.onpopstate = function () {
    history.go(1);
};
$(&quot;.date&quot;).on(&quot;dp.change&quot;, function() {
  var getdata = generateUmur();
  $('#umur_tahun').val(getdata['years']);
  $('#umur_bulan').val(getdata['months']);
  $('#umur_hari').val(getdata['days']);
});
$('#tanggal_lahir').keyup(function(){
  if($('#tanggal_lahir').val().length == 10){
    var getdata = generateUmur();
    $('#umur_tahun').val(getdata['years']);
    $('#umur_bulan').val(getdata['months']);
    $('#umur_hari').val(getdata['days']);
  }
});
$('#umur_tahun').keyup(function(){
  generateUmurToBirthday();  
});
$('#umur_bulan').keyup(function(){
  generateUmurToBirthday();
});
$('#umur_hari').keyup(function(){
  generateUmurToBirthday();
});

function generateUmurToBirthday()
{
  var bDay = moment().subtract($('#umur_tahun').val(), 'years');
  bDay.subtract($('#umur_bulan').val(), 'months');
  bDay.subtract($('#umur_hari').val(), 'days');
  bDay = bDay.format(&quot;DD-MM-YYYY&quot;);
  $('#tanggal_lahir').val(bDay);
}
$(&quot;input[name='cari_pasien'&quot;).autocomplete({
    delay: 1500,
    source: function( request, response ) {
        $.ajax({
            url: &quot;https://demo.epuskesmas.id/pasien/create&quot;,
            method: &quot;GET&quot;,
            dataType: &quot;json&quot;,
            data: {
                autocomplete: 'pasien',
                search: {'cari_pasien':request.term}
            },
            success: function(data) {
                response(data);
            },
            error: function (xhr, ajaxOptions, thrownError) {
                console.log(xhr.status);
                console.log(thrownError);
            }
        });
    },
    focus: function(event,ui){
        $(this).parent().removeClass('has-success').addClass('has-error');
        return false;
    },
    select: function(event,ui){
        $(this).parent().removeClass('has-error').addClass('has-success');
        getPasienData(ui.item);     
    },
    minLength: 5
});


$('.date').datetimepicker({
      format:'DD-MM-YYYY',
      useCurrent: false,
  });

function getPasienData(item)
{
    $.ajax({
        url: &quot;https://demo.epuskesmas.id/pasien/create&quot;,
        method: &quot;GET&quot;,
        dataType: &quot;json&quot;,
        data: {
            autocomplete: 'get_pasien',
            search: {'get_pasien':item.id}
        },
        success: function(data) {
            if(data.length > 0) {
                data.forEach(function(items,index){
                    setDataDariKota(items);
                });
                umur = generateUmur()
                $('#umur_tahun').val(umur['years']);
                $('#umur_bulan').val(umur['months']);
                $('#umur_hari').val(umur['days']);
            }
        },
        error: function (xhr, ajaxOptions, thrownError) {
            console.log(xhr.status);
            console.log(thrownError);
        }
    });
}

function generateUmur()
{
  var arr = [];
  var tes = $('#tanggal_lahir').val();
  var real = moment(&quot;&quot;+tes+&quot;&quot;, &quot;DD-MM-YYYY&quot;);
  var now = moment();
  var years = now.diff(real, 'years');
  real.add(years, 'years');

  var months = now.diff(real, 'months');
  real.add(months, 'months');

  var days = now.diff(real, 'days');

  arr['years'] = years;
  arr['months'] = months;
  arr['days'] = days;
  
  return arr;
}

function generateUmurByDate(data)
{
  var arr = [];
  var birthdate = moment(&quot;&quot;+data+&quot;&quot;, &quot;DD-MM-YYYY&quot;);
  var now = moment();

  var years = now.diff(birthdate, 'year');
  birthdate.add(years, 'years');

  var months = now.diff(birthdate, 'months');
  birthdate.add(months, 'months');

  var days = now.diff(birthdate, 'days');
  arr['years'] = years;
  arr['months'] = months;
  arr['days'] = days;
  
  return arr;
}
$(&quot;input[name='MPasien[no_kk]'&quot;).autocomplete({
    delay: 1000,
    source: function( request, response ) {
        $.ajax({
            url: &quot;https://demo.epuskesmas.id/pasien/create&quot;,
            method: &quot;GET&quot;,
            dataType: &quot;json&quot;,
            data: {
                autocomplete: 'no_kk',
                search: {'no_kk':request.term}
            },
            success: function(data) {
                response(data);
            },
            error: function (xhr, ajaxOptions, thrownError) {
                console.log(xhr.status);
                console.log(thrownError);
            }
        });
    },
    focus: function(event,ui){
        $(this).parent().removeClass('has-success').addClass('has-error');
        return false;
    },
    select: function(event,ui){
        $(this).parent().removeClass('has-error').addClass('has-success');
        setDataDariKK(ui.item);
    },
    minLength: 5
});

function setDataDariKota(item)
{
    setDataDariKK(item);
    $('#created_id').html(item.id);

    $('input[name=&quot;MPasien[verified_by]&quot;]').prop(&quot;checked&quot;, false);
    if(item.verified_by != &quot;&quot; &amp;&amp; item.verified_by != null){
        $('input[name=&quot;MPasien[nik]&quot;]').attr(&quot;required&quot;, false);
        $('input[name=&quot;MPasien[verified_by]&quot;]').prop(&quot;checked&quot;, true);
    }else{
        $('input[name=&quot;MPasien[nik]&quot;]').attr(&quot;required&quot;, true);
        $('input[name=&quot;MPasien[verified_by]&quot;]').prop(&quot;checked&quot;, false);
    }
    setRequiredForm(false);

    if(item.ktp_alamat_berbeda == 1 || item.ktp_alamat_berbeda == true){
        $('input[name=&quot;MPasien[ktp_propinsi_id]&quot;]').attr(&quot;required&quot;, false);
        if($('input[name=&quot;MPasien[ktp_alamat_berbeda]&quot;]').is(':not(:checked)')){
            $(&quot;#collapseAlamatKtp&quot;).collapse(&quot;show&quot;);
            $('input[name=&quot;MPasien[ktp_alamat_berbeda]&quot;]').prop(&quot;checked&quot;, true);
        }
    }else{
        $('input[name=&quot;MPasien[ktp_propinsi_id]&quot;]').attr(&quot;required&quot;, true);
        $('input[name=&quot;MPasien[ktp_alamat_berbeda]&quot;]').prop(&quot;checked&quot;, false);
        $('#collapseAlamatKtp').collapse(&quot;hide&quot;);
    }
    setRequiredFormKtp(false);

    $('select[name=&quot;MPasien[asuransi_id]&quot;]').val(item.asuransi_id).change();
    $('input[name=&quot;MPasien[no_asuransi]&quot;]').val(item.no_asuransi);
    $('input[name=&quot;MPasien[id]&quot;]').val(item.id);
    $('input[name=&quot;MPasien[nama]&quot;]').val(item.nama);
    $('input[name=&quot;MPasien[nik]&quot;]').val(item.nik);
    $('input[name=&quot;MPasien[no_rm_lama]&quot;]').val(item.no_rm_lama);
    $('input[name=&quot;MPasien[jenis_kelamin]&quot;][value=&quot;'+item.jenis_kelamin+'&quot;]').prop(&quot;checked&quot;, true);
    $('input[name=&quot;MPasien[tempat_lahir]&quot;]').val(item.tempat_lahir);
    $('input[name=&quot;MPasien[tanggal_lahir]&quot;]').val(item.tanggal_lahir);
    $('select[name=&quot;MPasien[gol_darah]&quot;]').val(item.gol_darah);
    $('input[name=&quot;MPasien[no_hp]&quot;]').val(item.no_hp);
    $('input[name=&quot;MPasien[pekerjaan_id]&quot;]').val(item.pekerjaan_id);
    $('input[name=&quot;MPasien[instansi]&quot;]').val(item.instansi);
    $('input[name=&quot;pekerjaan_nama&quot;]').val(item.pekerjaan_nama);
    $('select[name=&quot;MPasien[agama]&quot;]').val(item.agama);
    $('select[name=&quot;MPasien[pendidikan]&quot;]').val(item.pendidikan);
    $('select[name=&quot;MPasien[status_perkawinan]&quot;]').val(item.status_perkawinan);
    $('select[name=&quot;MPasien[status_keluarga]&quot;]').val(item.status_keluarga);
    $('select[name=&quot;MPasien[warganegara]&quot;]').val(item.warganegara);
    $('input[name=&quot;MPasien[nama_ibu]&quot;]').val(item.nama_ibu);

    $('input[name=&quot;MPasien[ktp_propinsi_id]&quot;]').val(item.ktp_propinsi_id);
    $('input[name=&quot;ktp_propinsi_nama&quot;]').val(item.ktp_propinsi_nama);
    $('input[name=&quot;MPasien[ktp_kota_id]&quot;]').val(item.ktp_kota_id);
    $('input[name=&quot;ktp_kota_nama&quot;]').val(item.ktp_kota_nama);
    $('input[name=&quot;MPasien[ktp_kecamatan_id]&quot;]').val(item.ktp_kecamatan_id);
    $('input[name=&quot;ktp_kecamatan_nama&quot;]').val(item.ktp_kecamatan_nama);
    $('input[name=&quot;MPasien[ktp_kelurahan_id]&quot;]').val(item.ktp_kelurahan_id);
    $('input[name=&quot;ktp_kelurahan_nama&quot;]').val(item.ktp_kelurahan_nama);
    $('input[name=&quot;MPasien[ktp_dusun_id]&quot;]').val(item.ktp_dusun_id);
    $('input[name=&quot;ktp_dusun_nama&quot;]').val(item.ktp_dusun_nama);
    $('textarea[name=&quot;MPasien[ktp_alamat]&quot;]').val(item.ktp_alamat);
    $('input[name=&quot;MPasien[ktp_rt]&quot;]').val(item.ktp_rt);
    $('input[name=&quot;MPasien[ktp_rw]&quot;]').val(item.ktp_rw);

    $('input[name=&quot;MPasien[imd]&quot;]').prop(&quot;checked&quot;, false);
    $('input[name=&quot;MPasien[bb_lahir]&quot;]').val(item.bb_lahir);
    $('input[name=&quot;MPasien[tb_lahir]&quot;]').val(item.tb_lahir);
    $('input[name=&quot;MPasien[imd]&quot;][value=&quot;'+ item.imd +'&quot;]').prop('checked', true);

}

function setDataDariKK(item)
{
    $('input[name=&quot;MPasien[propinsi_id]&quot;]').val(item.propinsi_id);
    $('input[name=&quot;propinsi_nama&quot;]').val(item.propinsi_nama);
    $('input[name=&quot;MPasien[kota_id]&quot;]').val(item.kota_id);
    $('input[name=&quot;kota_nama&quot;]').val(item.kota_nama);
    $('input[name=&quot;MPasien[kecamatan_id]&quot;]').val(item.kecamatan_id);
    $('input[name=&quot;kecamatan_nama&quot;]').val(item.kecamatan_nama);
    $('input[name=&quot;MPasien[kelurahan_id]&quot;]').val(item.kelurahan_id);
    $('input[name=&quot;kelurahan_nama&quot;]').val(item.kelurahan_nama);
    $('input[name=&quot;MPasien[dusun_id]&quot;]').val(item.dusun_id);
    $('input[name=&quot;dusun_nama&quot;]').val(item.dusun_nama);
    $('textarea[name=&quot;MPasien[alamat]&quot;]').val(item.alamat);
    $('input[name=&quot;MPasien[rt]&quot;]').val(item.rt);
    $('input[name=&quot;MPasien[rw]&quot;]').val(item.rw);
    $('input[name=&quot;MPasien[nama_ayah]&quot;]').val(item.nama_ayah);
}

$(&quot;input[name='pekerjaan_nama'&quot;).autocomplete({
    delay: 1000,
    source: function( request, response ) {
        $.ajax({
            url: &quot;https://demo.epuskesmas.id/pasien/create&quot;,
            method: &quot;GET&quot;,
            dataType: &quot;json&quot;,
            data: {
                autocomplete: 'pekerjaan',
                search: {'pekerjaan_nama':request.term}
            },
            success: function(data) {
                response(data);
            },
            error: function (xhr, ajaxOptions, thrownError) {
                console.log(xhr.status);
                console.log(thrownError);
            }
        });
    },
    focus: function(event,ui){
        $(this).parent().removeClass('has-success').addClass('has-error');
        return false;
    },
    select: function(event,ui){
        $(this).parent().removeClass('has-error').addClass('has-success');
        $('input[name=&quot;MPasien[pekerjaan_id]&quot;]').val(ui.item.id);
    },
    minLength: 3
});
$(&quot;input[name='pekerjaan_suami_nama'&quot;).autocomplete({
    delay: 1000,
    source: function( request, response ) {
        $.ajax({
            url: &quot;https://demo.epuskesmas.id/pasien/create&quot;,
            method: &quot;GET&quot;,
            dataType: &quot;json&quot;,
            data: {
                autocomplete: 'pekerjaan',
                search: {'pekerjaan_nama':request.term}
            },
            success: function(data) {
                response(data);
            },
            error: function (xhr, ajaxOptions, thrownError) {
                console.log(xhr.status);
                console.log(thrownError);
            }
        });
    },
    focus: function(event,ui){
        $(this).parent().removeClass('has-success').addClass('has-error');
        return false;
    },
    select: function(event,ui){
        $(this).parent().removeClass('has-error').addClass('has-success');
        $('input[name=&quot;MPasien[pekerjaan_suami_id]&quot;]').val(ui.item.id);
    },
    minLength: 3
});
$('input[name=&quot;MPasien[instansi]&quot;]').autocomplete({
    delay: 1000,
    source: function( request, response ) {
        $.ajax({
            url: &quot;https://demo.epuskesmas.id/pasien/create&quot;,
            method: &quot;GET&quot;,
            dataType: &quot;json&quot;,
            data: {
                autocomplete: 'instansi',
                search: {'instansi':request.term}
            },
            success: function(data) {
                response(data);
            },
            error: function (xhr, ajaxOptions, thrownError) {
                console.log(xhr.status);
                console.log(thrownError);
            }
        });
    },
    focus: function(event,ui){
        return false;
    },
    select: function(event,ui){
        $(this).parent().removeClass('has-error').addClass('has-success');
    },
    minLength: 3
});

function setWilayah(item)
{
    $('input[name=&quot;MPasien[propinsi_id]&quot;]').val(item.propinsi_id);
    $('input[name=propinsi_nama]').val(item.propinsi_nama);
    $('input[name=&quot;MPasien[kota_id]&quot;]').val(item.kota_id);
    $('input[name=kota_nama]').val(item.kota_nama);
    $('input[name=&quot;MPasien[kecamatan_id]&quot;]').val(item.kecamatan_id);
    $('input[name=kecamatan_nama]').val(item.kecamatan_nama);
    $('input[name=&quot;MPasien[kelurahan_id]&quot;]').val(item.kelurahan_id);
    $('input[name=kelurahan_nama]').val(item.kelurahan_nama);
    $('input[name=&quot;MPasien[dusun_id]&quot;]').val(item.dusun_id);
    $('input[name=dusun_nama]').val(item.dusun_nama);
}

function setWilayahKtp(item)
{
    $('input[name=&quot;MPasien[ktp_propinsi_id]&quot;]').val(item.propinsi_id);
    $('input[name=ktp_propinsi_nama]').val(item.propinsi_nama);
    $('input[name=&quot;MPasien[ktp_kota_id]&quot;]').val(item.kota_id);
    $('input[name=ktp_kota_nama]').val(item.kota_nama);
    $('input[name=&quot;MPasien[ktp_kecamatan_id]&quot;]').val(item.kecamatan_id);
    $('input[name=ktp_kecamatan_nama]').val(item.kecamatan_nama);
    $('input[name=&quot;MPasien[ktp_kelurahan_id]&quot;]').val(item.kelurahan_id);
    $('input[name=ktp_kelurahan_nama]').val(item.kelurahan_nama);
    $('input[name=&quot;MPasien[ktp_dusun_id]&quot;]').val(item.dusun_id);
    $('input[name=ktp_dusun_nama]').val(item.dusun_nama);
}

$(&quot;input[name='propinsi_nama'&quot;).autocomplete({
    delay: 1000,
    source: function( request, response ) {
        $.ajax({
            url: &quot;https://demo.epuskesmas.id/pasien/create&quot;,
            method: &quot;GET&quot;,
            dataType: &quot;json&quot;,
            data: {
                autocomplete: 'propinsi',
                search: {'propinsi_nama':request.term}
            },
            success: function(data) {
                response(data);
            },
            error: function (xhr, ajaxOptions, thrownError) {
                console.log(xhr.status);
                console.log(thrownError);
            }
        });
    },
    focus: function(event,ui){
        $(this).parent().removeClass('has-success').addClass('has-error');
        return false;
    },
    select: function(event,ui){
        setWilayah(ui.item);
        $('input[name=&quot;MPasien[propinsi_id]&quot;]').val(ui.item.id);
        $(this).parent().removeClass('has-error').addClass('has-success');
    },
    minLength: 3
});

$(&quot;input[name='kota_nama']&quot;).autocomplete({
    delay: 1000,
    source: function( request, response ) {
        $.ajax({
            url: &quot;https://demo.epuskesmas.id/pasien/create&quot;,
            method: &quot;GET&quot;,
            dataType: &quot;json&quot;,
            data: {
                autocomplete: 'kota',
                search: {
                    'propinsi_id':$('input[name=&quot;MPasien[propinsi_id]&quot;]').val(),
                    'kota_nama':request.term
                }
            },
            success: function(data) {
                response(data);
            },
            error: function (xhr, ajaxOptions, thrownError) {
                console.log(xhr.status);
                console.log(thrownError);
            }
        });
    },
    focus: function(event,ui){
        $(this).parent().removeClass('has-success').addClass('has-error');
        return false;
    },
    select: function(event,ui){
        setWilayah(ui.item);
        $('input[name=&quot;MPasien[kota_id]&quot;]').val(ui.item.id);
        $(this).parent().removeClass('has-error').addClass('has-success');
    },
    minLength: 3
});

$(&quot;input[name='kecamatan_nama']&quot;).autocomplete({
    delay: 1000,
    source: function( request, response ) {
        $.ajax({
            url: &quot;https://demo.epuskesmas.id/pasien/create&quot;,
            method: &quot;GET&quot;,
            dataType: &quot;json&quot;,
            data: {
                autocomplete: 'kecamatan',
                search: {
                    'kota_id':$('input[name=&quot;MPasien[kota_id]&quot;]').val(),
                    'kecamatan_nama':request.term
                }
            },
            success: function(data) {
                response(data);
            },
            error: function (xhr, ajaxOptions, thrownError) {
                console.log(xhr.status);
                console.log(thrownError);
            }
        });
    },
    focus: function(event,ui){
        $(this).parent().removeClass('has-success').addClass('has-error');
        return false;
    },
    select: function(event,ui){
        setWilayah(ui.item);
        $('input[name=&quot;MPasien[kecamatan_id]&quot;]').val(ui.item.id);
        $(this).parent().removeClass('has-error').addClass('has-success');
    },
    minLength: 3
});

$(&quot;input[name='kelurahan_nama']&quot;).autocomplete({
    delay: 1000,
    source: function( request, response ) {
        $.ajax({
            url: &quot;https://demo.epuskesmas.id/pasien/create&quot;,
            method: &quot;GET&quot;,
            dataType: &quot;json&quot;,
            data: {
                autocomplete: 'kelurahan',
                search: {
                    'kecamatan_id':$('input[name=&quot;MPasien[kecamatan_id]&quot;]').val(),
                    'kelurahan_nama':request.term
                }
            },
            success: function(data) {
                response(data);
            },
            error: function (xhr, ajaxOptions, thrownError) {
                console.log(xhr.status);
                console.log(thrownError);
            }
        });
    },
    focus: function(event,ui){
        $(this).parent().removeClass('has-success').addClass('has-error');
        return false;
    },
    select: function(event,ui){
        setWilayah(ui.item);
        $('input[name=&quot;MPasien[kelurahan_id]&quot;]').val(ui.item.id);
        $(this).parent().removeClass('has-error').addClass('has-success');
    },
    minLength: 3
});

$(&quot;input[name='dusun_nama']&quot;).autocomplete({
    delay: 1000,
    source: function( request, response ) {
        $.ajax({
            url: &quot;https://demo.epuskesmas.id/pasien/create&quot;,
            method: &quot;GET&quot;,
            dataType: &quot;json&quot;,
            data: {
                autocomplete: 'dusun',
                search: {
                    'kelurahan_id':$('input[name=&quot;MPasien[kelurahan_id]&quot;]').val(),
                    'dusun_nama':request.term
                }
            },
            success: function(data) {
                response(data);
            },
            error: function (xhr, ajaxOptions, thrownError) {
                console.log(xhr.status);
                console.log(thrownError);
            }
        });
    },
    focus: function(event,ui){
        $(this).parent().removeClass('has-success').addClass('has-error');
        return false;
    },
    select: function(event,ui){
        setWilayah(ui.item);
        $('input[name=&quot;MPasien[dusun_id]&quot;]').val(ui.item.id);
        $(this).parent().removeClass('has-error').addClass('has-success');
    },
    minLength: 3
});

$(&quot;input[name='ktp_propinsi_nama'&quot;).autocomplete({
    delay: 1000,
    source: function( request, response ) {
        $.ajax({
            url: &quot;https://demo.epuskesmas.id/pasien/create&quot;,
            method: &quot;GET&quot;,
            dataType: &quot;json&quot;,
            data: {
                autocomplete: 'propinsi',
                search: {'propinsi_nama':request.term}
            },
            success: function(data) {
                response(data);
            },
            error: function (xhr, ajaxOptions, thrownError) {
                console.log(xhr.status);
                console.log(thrownError);
            }
        });
    },
    focus: function(event,ui){
        $(this).parent().removeClass('has-success').addClass('has-error');
        return false;
    },
    select: function(event,ui){
        setWilayahKtp(ui.item);
        $('input[name=&quot;MPasien[ktp_propinsi_id]&quot;]').val(ui.item.id);
        $(this).parent().removeClass('has-error').addClass('has-success');
    },
    minLength: 3
});

$(&quot;input[name='ktp_kota_nama']&quot;).autocomplete({
    delay: 1000,
    source: function( request, response ) {
        $.ajax({
            url: &quot;https://demo.epuskesmas.id/pasien/create&quot;,
            method: &quot;GET&quot;,
            dataType: &quot;json&quot;,
            data: {
                autocomplete: 'kota',
                search: {
                    'propinsi_id':$('input[name=&quot;MPasien[ktp_propinsi_id]&quot;]').val(),
                    'kota_nama':request.term
                }
            },
            success: function(data) {
                response(data);
            },
            error: function (xhr, ajaxOptions, thrownError) {
                console.log(xhr.status);
                console.log(thrownError);
            }
        });
    },
    focus: function(event,ui){
        $(this).parent().removeClass('has-success').addClass('has-error');
        return false;
    },
    select: function(event,ui){
        setWilayahKtp(ui.item);
        $('input[name=&quot;MPasien[ktp_kota_id]&quot;]').val(ui.item.id);
        $(this).parent().removeClass('has-error').addClass('has-success');
    },
    minLength: 3
});

$(&quot;input[name='ktp_kecamatan_nama']&quot;).autocomplete({
    delay: 1000,
    source: function( request, response ) {
        $.ajax({
            url: &quot;https://demo.epuskesmas.id/pasien/create&quot;,
            method: &quot;GET&quot;,
            dataType: &quot;json&quot;,
            data: {
                autocomplete: 'kecamatan',
                search: {
                    'kota_id':$('input[name=&quot;MPasien[ktp_kota_id]&quot;]').val(),
                    'kecamatan_nama':request.term
                }
            },
            success: function(data) {
                response(data);
            },
            error: function (xhr, ajaxOptions, thrownError) {
                console.log(xhr.status);
                console.log(thrownError);
            }
        });
    },
    focus: function(event,ui){
        $(this).parent().removeClass('has-success').addClass('has-error');
        return false;
    },
    select: function(event,ui){
        setWilayahKtp(ui.item);
        $('input[name=&quot;MPasien[ktp_kecamatan_id]&quot;]').val(ui.item.id);
        $(this).parent().removeClass('has-error').addClass('has-success');
    },
    minLength: 3
});

$(&quot;input[name='ktp_kelurahan_nama']&quot;).autocomplete({
    delay: 1000,
    source: function( request, response ) {
        $.ajax({
            url: &quot;https://demo.epuskesmas.id/pasien/create&quot;,
            method: &quot;GET&quot;,
            dataType: &quot;json&quot;,
            data: {
                autocomplete: 'kelurahan',
                search: {
                    'kecamatan_id':$('input[name=&quot;MPasien[ktp_kecamatan_id]&quot;]').val(),
                    'kelurahan_nama':request.term
                }
            },
            success: function(data) {
                response(data);
            },
            error: function (xhr, ajaxOptions, thrownError) {
                console.log(xhr.status);
                console.log(thrownError);
            }
        });
    },
    focus: function(event,ui){
        $(this).parent().removeClass('has-success').addClass('has-error');
        return false;
    },
    select: function(event,ui){
        setWilayahKtp(ui.item);
        $('input[name=&quot;MPasien[ktp_kelurahan_id]&quot;]').val(ui.item.id);
        $(this).parent().removeClass('has-error').addClass('has-success');
    },
    minLength: 3
});

$(&quot;input[name='ktp_dusun_nama']&quot;).autocomplete({
    delay: 1000,
    source: function( request, response ) {
        $.ajax({
            url: &quot;https://demo.epuskesmas.id/pasien/create&quot;,
            method: &quot;GET&quot;,
            dataType: &quot;json&quot;,
            data: {
                autocomplete: 'dusun',
                search: {
                    'kelurahan_id':$('input[name=&quot;MPasien[ktp_kelurahan_id]&quot;]').val(),
                    'dusun_nama':request.term
                }
            },
            success: function(data) {
                response(data);
            },
            error: function (xhr, ajaxOptions, thrownError) {
                console.log(xhr.status);
                console.log(thrownError);
            }
        });
    },
    focus: function(event,ui){
        $(this).parent().removeClass('has-success').addClass('has-error');
        return false;
    },
    select: function(event,ui){
        setWilayahKtp(ui.item);
        $('input[name=&quot;MPasien[ktp_dusun_id]&quot;]').val(ui.item.id);
        $(this).parent().removeClass('has-error').addClass('has-success');
    },
    minLength: 3
});

function setDataDariApiBpjs(peserta)
{
    var style = peserta.ketAktif == &quot;AKTIF&quot; ? &quot;text-success&quot; : &quot;text-danger&quot;;
    var status = &quot;text-danger&quot;;
    if(peserta.status == 'Sesuai Faskes'){
        status = &quot;text-success&quot;;
    }else if(peserta.status == 'Faskes Jejaring'){
        status = &quot;text-info&quot;;
    }
    html = 'Status: &lt;b>&lt;span class='+style+'>'+peserta.ketAktif+'&lt;\/span>&lt;\/b>&lt;br>'
            +'Kode Provider: '+peserta.kdProviderPst.kdProvider+'&lt;br>'
            +'Nama Provider: '+peserta.kdProviderPst.nmProvider+'&lt;br>'
            +'Nama Peserta: '+(peserta.nama).toUpperCase()+'&lt;br>'
            +'Kelas : '+peserta.jnsKelas.nama+'&lt;br>'
            +'Jenis Peserta : '+peserta.jnsPeserta.nama+'&lt;br>'
            +'Prolanis/PRB : '+(peserta.pstProl != &quot;&quot; ? 'Ya('+peserta.pstProl+')' : &quot;Bukan&quot;)+(peserta.pstPrb != &quot;&quot; ? '/Ya('+peserta.pstPrb+')' : &quot;/Bukan&quot;)+'&lt;br>'
            +'Tunggakan: '+peserta.tunggakan+'&lt;br>'
            +'&lt;b>&lt;span class='+status+'>'+peserta.status+'&lt;\/span>&lt;\/b>&lt;br>';
    $('#data_peserta_bpjs').html(html);


    $('input[name=&quot;MPasien[no_asuransi]&quot;]').val(peserta.noKartu);
    if(peserta.noKTP != null){
        if(peserta.noKTP.trim() != &quot;&quot;){
            $('input[name=&quot;MPasien[nik]&quot;]').val(peserta.noKTP);
        }
    }
    $('input[name=&quot;MPasien[nama]&quot;]').val(peserta.nama);
    $('input[name=&quot;MPasien[jenis_kelamin]&quot;][value=&quot;'+peserta.sex+'&quot;]').attr(&quot;checked&quot;,true);
    $('input[name=&quot;MPasien[tanggal_lahir]&quot;]').val(peserta.tglLahir);
    $('select[name=&quot;MPasien[gol_darah]&quot;]').val(peserta.golDarah);
    $('input[name=&quot;MPasien[no_hp]&quot;]').val(peserta.noHP);
    $('select[name=&quot;MPasien[status_keluarga]&quot;]').val(peserta.hubunganKeluarga.toUpperCase());
    if(peserta.tglLahir != &quot;&quot;){
      var tgl = moment(&quot;&quot;+peserta.tglLahir+&quot;&quot;, &quot;DD-MM-YYYY&quot;).format(&quot;DD-MM-YYYY&quot;);
      var date = generateUmurByDate(tgl);
      $('#umur_tahun').val(date['years']);
      $('#umur_bulan').val(date['months']);
      $('#umur_hari').val(date['days']);
    }
}

function setAsuransi()
{
    var asuransi_id = $('select[name=&quot;MPasien[asuransi_id]&quot;]');
    var is_bridgingbpjs = asuransi_id.find(':selected').attr('is_bridgingbpjs');
    var is_bridging_mandiri_inhealth = asuransi_id.find(':selected').attr('is_bridging_mandiri_inhealth');
    var show_no_asuransi = asuransi_id.find(':selected').attr('show_no_asuransi');
    var require_no_asuransi = asuransi_id.find(':selected').attr('require_no_asuransi');

    $('#nokainhealt').addClass('hidden');
    $('#asuransi_bpjs').removeClass('hidden');

    if(show_no_asuransi == &quot;0&quot;){
        $('.no-asuransi').addClass('hidden');
        $('.no-asuransi input').removeAttr('required');
    }else{
        $('.no-asuransi').removeClass('hidden');
        if(require_no_asuransi == &quot;1&quot;){
            $('.icon_asuransi').removeClass('hidden');
            $('.no-asuransi input').attr('required', true);
        }else{
            $('.icon_asuransi').addClass('hidden');
            $('.no-asuransi input').attr('required', false);
        }
        
        $('#nokainhealt').attr('required', false);
        if(is_bridgingbpjs == &quot;1&quot;){
            $('#data_peserta_bpjs').removeClass('hidden');
            $('#button_bridgingbpjs').removeClass('hidden');
        }else{
            $('#data_peserta_bpjs').addClass('hidden');
            $('#button_bridgingbpjs').addClass('hidden');
            if (is_bridging_mandiri_inhealth == &quot;1&quot;) {
                if (isPenggunaMandiriInhealth == &quot;&quot; || isPenggunaMandiriInhealth == &quot;0&quot;) {
                    $('.no-asuransi').addClass('hidden');
                    $('.no-asuransi input').removeAttr('required');
                } else {
                    $('input[name=&quot;MPasien[no_asuransi]&quot;]').val(&quot;&quot;);
                    $('.no-asuransi input').attr('required', false);
                    if(require_no_asuransi == &quot;1&quot;) {
                        $('#nokainhealt').attr('required', true);
                    }

                    $('#asuransi_bpjs').addClass('hidden');
                    $('#nokainhealt').removeClass('hidden');
                }
            }
        }
    }
}

function checkAfter(){
    checkNoAsuransi();
    setTimeout(function() {
        setPesertaBpjs();
    }, 1000);
}

function setPesertaBpjs()
{
    var no_asuransi = $('input[name=&quot;MPasien[no_asuransi]&quot;]');
    if(no_asuransi.val() == ''){
        return false;
    }
    var nik = $('input[name=&quot;MPasien[nik]&quot;]');
    if(no_asuransi.val() == &quot;&quot; &amp;&amp; nik.val() == &quot;&quot;){
        alert('Silahkan isi &lt;b>No Asuransi&lt;/b> atau &lt;b>NIK&lt;/b> untuk mengecek kepesertaan BPJS!');
        no_asuransi.parent().addClass('has-error');
        nik.parent().addClass('has-error');
        return false;
    }else{
        no_asuransi.parent().removeClass('has-error');
        search = no_asuransi.val();
        if(search == &quot;&quot;){
            search = nik.val();
        }
    }
    no_asuransi.addClass('ui-autocomplete-loading');
    var asuransi_id = $('select[name=&quot;MPasien[asuransi_id]&quot;]').val();

    $.ajax({
        url: &quot;https://demo.epuskesmas.id/pasien/create&quot;,
        method: &quot;GET&quot;,
        dataType: &quot;json&quot;,
        data: {
            api: 'pesertabpjs',
            search: search,
            asuransi_id: asuransi_id,
        },
        success: function(data) {
            let is_bridgingbpjs_nonactive = data.is_bridgingbpjs_nonactive ?? &quot;false&quot;;
            if (is_bridgingbpjs_nonactive == &quot;active&quot;){

                alert('Mohon Maaf Settingan Sinkron Asuransi Tersebut sedang Non-aktif Tidak Bisa melakukan Pengecekan No. Asuransi','warning')
                return false;
            }

            no_asuransi.removeClass('ui-autocomplete-loading');
            if(data.hasOwnProperty('metaData')){
                if(data.metaData.code == 200){
                    peserta = data.response;
                    if(peserta.noKTP == null){
                        alert(&quot;&lt;b>No. KTP (NIK)&lt;\/b> peserta BPJS ini bernilai &lt;b>null&lt;\/b> di response Web Service.&quot; 
                            +&quot;&lt;br> 1. Pastikan pasien BPJS ini tidak memiliki kartu / nomor &lt;b>peserta BPJS ganda&lt;\/b> dan sudah melunasi pembayaran &lt;b>premi&lt;\/b> bulan ini.&quot;
                            +&quot;&lt;br> 2. Infokan kepada pasien BPJS ini agar melakukan cek dan pembaharuan &lt;b>data peserta&lt;\/b> BPJS ke kantor BPJS.&quot;
                            +&quot;&lt;br> 3. Anda tetap bisa mendaftarkan pasien BPJS ini sebagai pasien, namun pastikan untuk mengecek &lt;b>status bpjs&lt;\/b> di aplikasi ePuskesmas dan &lt;b>mengecek data di PCare&lt;\/b>.&quot;
                            +&quot;&lt;br> 4. Info selengkapnya silahkan hubungi &lt;b>petugas BPJS&lt;\/b>.&quot;, &quot;warning&quot;, 0);
                    }
                    alert(&quot;Data peserta BPJS ditemukan!&quot;, &quot;success&quot;);
                    setDataDariApiBpjs(peserta);
                    no_asuransi.parent().removeClass('has-error').addClass('has-success');
                    if($(&quot;#normcheck&quot;).val() == ''){
                        $('input[name=&quot;MPasien[nik]&quot;]').trigger('blur');
                        if($('input[name=&quot;MPasien[nik]&quot;]').val() != ''){
                            alert('Pengecekan data pasien di sistem selesai, silahkan cek kembali isian Anda  ..', 'success');
                        }
                    }
                }else{
                    if(data.metaData.code == 401){
                        alert(&quot;Anda tidak diperkenankan terkoneksi ke Webservice BPJS. Pastikan username dan password di &lt;b>Pengaturan - BPJS&lt;\/b> disesuaikan dengan username dan password PCare terbaru!&lt;br> &lt;span class='text-danger'> Kode Error: &lt;b>&quot;+data.metaData.code+&quot;&lt;\/b>&lt;\/span>&quot;, &quot;warning&quot;);
                    }else{
                        alert(&quot;Data peserta BPJS tidak ditemukan! Cek kembali &lt;b>No Asuransi&lt;\/b> yang diinput &lt;b>atau&lt;\/b> tunggu beberapa saat hingga Webservice BPJS kembali normal.&lt;br> &lt;span class='text-danger'> Kode Error: &lt;b>&quot;+data.metaData.code+&quot;&lt;\/b>&lt;\/span>&quot;, &quot;warning&quot;);
                    }
                    no_asuransi.parent().addClass('has-error');
                    nik.parent().addClass('has-error');
                    $('#data_peserta_bpjs').html(&quot;&quot;);
                }
            }else{
                if(data.hasOwnProperty('o2o')){
                    alert(data.o2o.message, data.o2o.status);
                }else{
                    alert(&quot;Data peserta BPJS tidak ditemukan! Cek kembali &lt;b>No Asuransi&lt;\/b> yang diinput &lt;b>&quot;, &quot;warning&quot;);
                }
                no_asuransi.parent().addClass('has-error');
                nik.parent().addClass('has-error');
                $('#data_peserta_bpjs').html(&quot;&quot;);
            }
        },
        error: function (xhr, ajaxOptions, thrownError) {
            console.log(xhr.status);
            console.log(thrownError);
            no_asuransi.removeClass('ui-autocomplete-loading');
        }
    });
}

function setRequiredForm(reset = false, show = 0)
{
    var requiredInput = [
                        'nik','tempat_lahir','tanggal_lahir',
                        'propinsi_id','kota_id','kecamatan_id','kelurahan_id', 'alamat', 'rt', 'rw',
                        'pekerjaan_id','agama','pendidikan','status_perkawinan','status_keluarga','warganegara','no_hp'
                        ];

    if(global_required!='undefined'){
        global_required.forEach(function(item,index){
            for (var i=0; i&lt;requiredInput.length; i++)
                if (requiredInput[i] === item){
                    delete requiredInput[i]; 
                }                                    
        });
    }

    if(reset == false){
        requiredInput.forEach(function(item,index){
            var obj = $('[name=&quot;MPasien['+item+']&quot;]');
            obj.attr('required',true);
            obj.parents('.form-group').find('span[style]').remove();
            obj.parents('.form-group').find('label').append(' &lt;span style=&quot;color:red;&quot;>*&lt;/span>');
        });    
    }else{
        requiredInput.forEach(function(item,index){
            var obj = $('[name=&quot;MPasien['+item+']&quot;]');            
            obj.removeAttr('required');
            obj.parents('.form-group').find('span[style]').remove();
        });
    }
    
}

function setRequiredFormKtp(reset = false)
{
    var requiredInput = [
                        'ktp_propinsi_id','ktp_kota_id','ktp_kecamatan_id','ktp_kelurahan_id', 'ktp_alamat', 'ktp_rt', 'ktp_rw'
                        ];
    set_up = $('[name=&quot;MPasien['+requiredInput[0]+']&quot;]').is('[required]');
    if(reset==true){
        $('#collapseAlamatKtp.collapse').removeClass('in');
    }
    if(set_up == false &amp;&amp; reset == false){
        requiredInput.forEach(function(item,index){
            var obj = $('[name=&quot;MPasien['+item+']&quot;]');
            obj.attr('required',true);
            obj.parents('.form-group').find('span[style]').remove();
            obj.parents('.form-group').find('label').append(' &lt;span style=&quot;color:red;&quot;>*&lt;/span>');
        });    
    }else{
        $('[name=&quot;MPasien[ktp_alamat_berbeda]&quot;]').removeAttr('checked');
        requiredInput.forEach(function(item,index){
            var obj = $('[name=&quot;MPasien['+item+']&quot;]');
            obj.removeAttr('required');
            obj.parents('.form-group').find('span[style]').remove();
        });
    }
}

let isPenggunaMandiriInhealth = &quot;&quot;;

$(&quot;form[form-ajax-print]&quot;).each(function(){
    $(this).on('submit',function(event){
        event.preventDefault();
        var button_submit = $(this).find('button[type=submit]');
        var button_print = $(this).find('button[id=button_print]');
        var button_print_kartu = $(this).find('button[id=button_print_kartu]');
        var button_reset = $(this).find('button[type=reset]');
        button_submit.addClass('loading');

        if(isPenggunaMandiriInhealth == &quot;1&quot;) 
        {
            var is_bridging_mandiri_inhealth = $('select[name=&quot;MPasien[asuransi_id]&quot;]').find(':selected').attr('is_bridging_mandiri_inhealth');

            if (is_bridging_mandiri_inhealth == &quot;1&quot;) {
                let nokainhealt = $('input[name=&quot;nokainhealt&quot;]').val();
                $('input[name=&quot;MPasien[no_asuransi]&quot;]').val(nokainhealt);                
            }

        }

        $('#button_saves').addClass('loading');
        for (instance in CKEDITOR.instances) {
            CKEDITOR.instances[instance].updateElement();
        }
                
        unsetSeparatorNumber();
        $.ajax({
            url: $(this).attr('action'),
            method: $(this).attr('method'),
            dataType: &quot;json&quot;,
            data: $(this).serialize(),
            success: function(data) {
                if(data.activeSwal == &quot;true&quot;){
                    button_submit.removeClass('loading');
                    $(&quot;#button_saves&quot;).removeClass('loading');
                    Swal.fire({ 
                      title: 'Apakah Anda Yakin Merubah Data ?',
                      type: 'info',
                      customClass: 'swal-wide-pasien',
                      showCancelButton: true,
                      confirmButtonColor: '#3085d6',
                      cancelButtonColor: '#d33',
                      confirmButtonText: 'Ya',
                      cancelButtonText: 'Tidak',
                      html: data.message,
                    }).then((result) => {
                      if (result.value) {
                        $(&quot;#cekBpjs&quot;).val(&quot;false&quot;);
                        $(&quot;#button_save&quot;).click();
                      }else if (result.dismiss == &quot;cancel&quot;){
                        $(&quot;#cekBpjs&quot;).val(&quot;default&quot;);
                      }else{
                        $(&quot;#cekBpjs&quot;).val(&quot;default&quot;);
                      }
                      
                    });
                    return false;
                }

                alert(data.message, data.status);
                button_submit.removeClass('loading');
                $('#button_saves').removeClass('loading');
                if(data.status == 'success'){
                    syncPasienKiosk();
                    button_submit.hide();
                    $('#button_saves').hide();
                    button_print.show();
                    button_print_kartu.show();
                    button_reset.click(function(){
                        button_submit.show();
                        $('#button_saves').show();
                        button_print.hide();
                        button_print.attr('onclick','');
                        button_print_kartu.hide();
                        button_print_kartu.attr('onclick','');
                        $( &quot;label[id=created_id]&quot; ).html('- Otomatis -');
                    });
                    if(data.id != undefined &amp;&amp; data.id != ''){
                        $( &quot;label[id=created_id]&quot; ).html(data.id);
                    }
                    if(data.next_url != undefined &amp;&amp; data.next_url != ''){
                        $('#button_next').attr('href',data.next_url);
                        $('#button_next').show();
                    }
                    if(data.print_url != undefined &amp;&amp; data.print_url != ''){
                        button_print.attr('onclick','window.open(&quot;'+data.print_url+'&quot;, &quot;&quot;, &quot;left=100,top=100&quot;);');
                    }

                    if($('#askep_id') != null){
                        $('#button_askep').attr('href', data.next_url+'?pengkajian_keperawatan_id='+$('#askep_id').val());
                    }

                    if(data.print_url_kartu != undefined &amp;&amp; data.print_url_kartu != ''){
                        button_print_kartu.attr('onclick','window.open(&quot;'+data.print_url_kartu+'&quot;, &quot;&quot;, &quot;left=100,top=100,width=420,height=260&quot;);');
                    }

                    if(data.show_url != undefined &amp;&amp; data.show_url != ''){
                        var url = data.show_url;
                        $.pjax({url: url, container: '#content'});
                    }
                }
                setSeparatorNumber();
            },
            error: function (xhr, ajaxOptions, thrownError) {
                alert(&quot;Terjadi kesalahan sistem, silahkan hubungi team support kami!&quot;, &quot;danger&quot;);
                button_submit.removeClass('loading');
                $('#button_saves').removeClass('loading');
                console.log(xhr.status);
                console.log(thrownError);
                setSeparatorNumber();
            }
        });
    });
});

function checkNoAsuransi(){
    checkAsuransiVal();
    var no_asuransi = $('input[name=&quot;MPasien[no_asuransi]&quot;]').val();
    var asuransi_id = $('select[name=&quot;MPasien[asuransi_id]&quot;]').val();
    $.ajax({
        url: &quot;https://demo.epuskesmas.id/pasien/create&quot;,
        method: &quot;GET&quot;,
        dataType: &quot;json&quot;,
        data: {
            check: 'no_asuransi',
            search: {
                'no_asuransi':no_asuransi,
                'asuransi_id':asuransi_id,
                'id':''
            }
        },
        success: function(data) {
            if(data.status == 'success'){
                $('input[name=&quot;MPasien[no_asuransi]&quot;]').parent().addClass('has-success').removeClass('has-error');
            }else if(data.length &lt;= 0){
                $('input[name=&quot;MPasien[no_asuransi]&quot;]').parent().addClass('has-error').removeClass('has-success');
            }else{
                alert(data.message, data.status);
                $('input[name=&quot;MPasien[no_asuransi]&quot;]').parent().addClass('has-error').removeClass('has-success').attr('required');
                $('input[name=&quot;MPasien[no_asuransi]&quot;]').val('');
                $('#data_peserta_bpjs').html('');
            }
            
        },
        error: function (xhr, ajaxOptions, thrownError) {
            console.log(xhr.status);
            console.log(thrownError);
            $('input[name=&quot;MPasien[no_asuransi]&quot;]').parent().removeClass('has-success').addClass('has-error');
        }
    });

}

function checkNik(nik){
    if(&quot;e-Puskesmas&quot; == 'e-Clinic') {
      var Nik = $(&quot;input[name='MPasien[nik]']&quot;);
      Nik.attr('required', true); 
      Nik.parents('.form-group').find('span[style]').remove();
      Nik.parents('.form-group').find('label').append(' &lt;span style=&quot;color:red;&quot;>*&lt;/span>');
      var radio = $('#radioNik');
      radio.attr('required', false);
      radio.prop('checked', false);
      radio.attr('checked', false);
      radio.parent('label').find('span[style]').remove();
    }
    if(nik.length != 16){
        var message = &quot;&quot;;
        if(nik.length &lt; 16){
            message += &quot;NIK kurang dari 16 digit!&quot;;
        }else{
            message += &quot;NIK lebih dari 16 digit!&quot;;
        }
        message +=&quot;&lt;br>Silahkan periksa kembali dan pastikan NIK yang diinput sesuai dengan yang tercantum di KTP atau KK!&quot;;
        alert(message);
        $('input[name=&quot;MPasien[nik]&quot;]').parent().addClass('has-error').removeClass('has-success');
        return false;
    }
        $.ajax({
        url: &quot;https://demo.epuskesmas.id/pasien/create&quot;,
        method: &quot;GET&quot;,
        dataType: &quot;json&quot;,
        data: {
            check: 'nik',
            search: {
                'nik':nik,
                'id':''
            }
        },
        success: function(data) {
                            if(data.puskesmas == true){
                    window.location = &quot;https://demo.epuskesmas.id/pasien/edit/&quot;+data.result.id;
                }else{
                    if(data.result){
                        setDataDariKota(data.result);
                    }
                }
                return false;
                        if(data.status == 'success'){
                $('input[name=&quot;MPasien[nik]&quot;]').parent().addClass('has-success').removeClass('has-error');
            }else{
                alert(data.message, data.status);
                $('input[name=&quot;MPasien[nik]&quot;]').parent().addClass('has-error').removeClass('has-success');
            }
        },
        error: function (xhr, ajaxOptions, thrownError) {
            console.log(xhr.status);
            console.log(thrownError);
            $('input[name=&quot;MPasien[nik]&quot;]').parent().removeClass('has-success').addClass('has-error');
        }
    });
}

function getDisdukcapil(){
    var nik = $('input[name=&quot;MPasien[nik]&quot;]');
    $.ajax({
        url: &quot;https://demo.epuskesmas.id/pasien/create&quot;,
        method: &quot;GET&quot;,
        dataType: &quot;json&quot;,
        data: {
            getData: 'disdukcapil',
            search: {
                'nik':nik.val(),
            }
        },
        success: function(data) {
            setRequiredForm(true);
            $('input[name=&quot;MPasien[verified_by]&quot;]').prop(&quot;checked&quot;, false);
            if(data.kesalahan){
                alert(data.kesalahan);
                $('input[name=&quot;MPasien[nik]&quot;]').parent().addClass('has-error').removeClass('has-success');
            }else{
                alert(&quot;Biodata Disdukcapil ditemukan!&quot;, &quot;success&quot;);
                setRequiredForm(false);
                $('input[name=&quot;MPasien[verified_by]&quot;]').prop(&quot;checked&quot;, true);
                setDataDisdukcapil(data);
                $('input[name=&quot;MPasien[nik]&quot;]').parent().addClass('has-success').removeClass('has-error');
            }
        },
        error: function (xhr, ajaxOptions, thrownError) {
            console.log(xhr.status);
            console.log(thrownError);
            $('input[name=&quot;MPasien[nik]&quot;]').parent().removeClass('has-success').addClass('has-error');
        }
    });
}


function saveAtas(){
    $('#button_saves').addClass('loading');
    $('#button_save').click();
}

function setDataDisdukcapil(data)
{
    setWilayah(data);
    $('input[name=&quot;MPasien[nama]&quot;]').val(data.nama);
    $('input[name=&quot;MPasien[jenis_kelamin]&quot;][value=&quot;'+data.jenis_kelamin+'&quot;]').prop(&quot;checked&quot;,true);
    $('input[name=&quot;MPasien[tanggal_lahir]&quot;]').val(data.tanggal_lahir);
    $('select[name=&quot;MPasien[gol_darah]&quot;]').val(data.gol_darah);
    $('input[name=&quot;MPasien[no_hp]&quot;]').val(data.no_hp);
    $('select[name=&quot;MPasien[status_keluarga]&quot;]').val(data.status_keluarga);
    $('select[name=&quot;MPasien[status_perkawinan]&quot;]').val(data.status_perkawinan);
    $('select[name=&quot;MPasien[pendidikan]&quot;]').val(data.pendidikan);
    $('select[name=&quot;MPasien[agama]&quot;]').val(data.agama);
    $('input[name=&quot;pekerjaan_nama&quot;]').val(data.pekerjaan_nama);
    $('input[name=&quot;MPasien[pekerjaan_id]&quot;]').val(data.pekerjaan_id);
    $('input[name=&quot;MPasien[instansi]&quot;]').val(data.instansi);
    $('textarea[name=&quot;MPasien[alamat]&quot;]').val(data.alamat);
    $('input[name=&quot;MPasien[rt]&quot;]').val(data.rt);
    $('input[name=&quot;MPasien[rw]&quot;]').val(data.rw);
    $('input[name=&quot;MPasien[nama_ayah]&quot;]').val(data.nama_ayah);
    $('input[name=&quot;MPasien[nama_ibu]&quot;]').val(data.nama_ibu);
    $('input[name=&quot;MPasien[tempat_lahir]&quot;]').val(data.tempat_lahir);
    $('input[name=&quot;MPasien[no_kk]&quot;]').val(data.no_kk);
    $('select[name=&quot;MPasien[warganegara]&quot;]').val(data.warganegara);
}

    var global_required = [&quot;&quot;] ;


$(document).ready(function(){

        setAsuransi();


    if($('#checked_verifikasi').is(':checked')) {
        setRequiredForm(false);
    }else{
        setRequiredForm(true);
    }

    $(&quot;input[name='MPasien[no_dok_rm]']&quot;).on('blur', function () {
        $.ajax({
            url: &quot;https://demo.epuskesmas.id/pasien/create&quot;,
            method: &quot;GET&quot;,
            dataType: &quot;json&quot;,
            data: {
                no_dok_rm: this.value
            },
            success: function(data) {
                if (data.message != '') {
                    Swal.fire({ 
                        title: 'Perhatian!',
                        type: 'info',
                        customClass: 'swal-wide-pasien',
                        showCancelButton: true,
                        confirmButtonColor: '#3085d6',
                        cancelButtonColor: '#d33',
                        confirmButtonText: 'Ya',
                        cancelButtonText: 'Tidak',
                        html: data.message,
                    }).then((result) => {
                        if (result.value) {
                            $(&quot;input[name='MPasien[no_dok_rm]']&quot;).val('').focus();
                        }
                    });
                }
            },
            error: function (xhr, ajaxOptions, thrownError) {
                console.log(xhr.status);
                console.log(thrownError);
            }
        });
    });
});

$('#checked_verifikasi').click( function(){
    if($(this).is(':checked')) {
        setRequiredForm(false);
    }else{
        setRequiredForm(true);
    }
});

function checkAsuransiVal(){
    var asuransi_id = $('select[name=&quot;MPasien[asuransi_id]&quot;]');
    var is_bridgingbpjs = asuransi_id.find(':selected').attr('is_bridgingbpjs');
    var no_asuransi = $('input[name=&quot;MPasien[no_asuransi]&quot;]').val();
    var pad = '';
    if (is_bridgingbpjs == '1' &amp;&amp; no_asuransi.length &lt; 13 &amp;&amp; no_asuransi != '') {
        for (i=1; i &lt;= (13 - no_asuransi.length); i++) {
            pad += '0';
        }
    }
    $('input[name=&quot;MPasien[no_asuransi]&quot;]').val(`${pad}${no_asuransi}`);
}

function printLabel(id, tipe) {
    window.open(`https://demo.epuskesmas.id/pasien/printlabel/${id}/${tipe}`, '', 'left=0,top=0');
}

$(&quot;select.warganegara&quot;).change(function(){
        var selected = $(this).children(&quot;option:selected&quot;).val();
        if (selected == &quot;INDONESIA&quot;) {
            $(&quot;input[name='MPasien[negara_asal]']&quot;).val('INDONESIA');
            if (&quot;e-Puskesmas&quot; == 'e-Clinic') {
              var paspor = $(&quot;input[name='MPasien[no_paspor]']&quot;);
              paspor.attr('required', false);
              paspor.parents('.form-group').find('span[style]').remove();
              paspor.parents('.form-group').addClass('hidden');
              var nik = $(&quot;input[name='MPasien[nik]']&quot;);
              nik.parents('.form-group').removeClass('hidden');
              nik.attr('required', true);
              nik.parents('.form-group').find('span[style]').remove();
              nik.parents('.form-group').find('label').append(' &lt;span style=&quot;color:red;&quot;>*&lt;/span>');
              var radnik = $('#radioNik');
              radnik.parents('.form-group').removeClass('hidden');
              setRequiredFormAlamat(true);
            }
        } else if (selected == &quot;ASING&quot;){
            $(&quot;input[name='MPasien[negara_asal]']&quot;).val('');
            if (&quot;e-Puskesmas&quot; == 'e-Clinic') {
              var paspor = $(&quot;input[name='MPasien[no_paspor]']&quot;);
              paspor.parents('.form-group').removeClass('hidden');
              paspor.parents('.form-group').find('label').append(' &lt;span style=&quot;color:red;&quot;>*&lt;/span>');
              paspor.attr('required', true);
              var nik = $(&quot;input[name='MPasien[nik]']&quot;);
              nik.val('');
              nik.attr('required', false);
              nik.parents('.form-group').find('span[style]').remove();
              nik.parents('.form-group').addClass('hidden');
              var radnik = $('#radioNik');
              radnik.attr('required', false);
              radnik.attr('checked', false);
              radnik.prop('checked', false);
              radnik.parents('.radio-inline').find('span[style]').remove();
              radnik.parents('.form-group').addClass('hidden');
              setRequiredFormAlamat(false);
            }
        } else {
            $(&quot;input[name='MPasien[negara_asal]']&quot;).val('');
            if (&quot;e-Puskesmas&quot; == 'e-Clinic') {
              var paspor = $(&quot;input[name='MPasien[no_paspor]']&quot;);
              paspor.attr('required', false);
              paspor.parents('.form-group').find('span[style]').remove();
              paspor.parents('.form-group').addClass('hidden');
              var nik = $(&quot;input[name='MPasien[nik]']&quot;);
              nik.val('');
              nik.attr('required', false);
              nik.parents('.form-group').find('span[style]').remove();
              nik.parents('.form-group').addClass('hidden');
              var radnik = $('#radioNik');
              radnik.attr('required', false);
              radnik.attr('checked', false);
              radnik.prop('checked', false);
              radnik.parents('.radio-inline').find('span[style]').remove();
              radnik.parents('.form-group').addClass('hidden');
              setRequiredFormAlamat(false);
            }
        }
    });

function checkradionik(obj)
{
  var radio = $('#radioNik');
  var nik = $(&quot;input[name='MPasien[nik]']&quot;);
  if ($(obj).is(':checked')) {
    nik.val(&quot;&quot;);
    nik.attr('required', false);
    nik.parents('.form-group').find('span[style]').remove();
    radio.attr('checked', true);
  } else {
    radio.attr('checked', false);
    nik.val(&quot;&quot;);
    nik.attr('required', true);
    nik.parents('.form-group').find('span[style]').remove();
    nik.parents('.form-group').find('label').append(' &lt;span style=&quot;color:red;&quot;>*&lt;/span>');
  }
}

function setRequiredFormAlamat(reset = false, show = 0)
{
  var requiredInput = ['propinsi_id','kota_id','kecamatan_id','kelurahan_id', 'alamat', 'rt', 'rw'];
  if(reset == true){
  requiredInput.forEach(function(item,index){
    var obj = $('[name=&quot;MPasien['+item+']&quot;]');
    obj.attr('required',true);
    obj.parents('.form-group').find('span[style]').remove();
    obj.parents('.form-group').find('label').append(' &lt;span style=&quot;color:red;&quot;>*&lt;/span>');
  });    
  }else{
    requiredInput.forEach(function(item,index){
      var obj = $('[name=&quot;MPasien['+item+']&quot;]');            
      obj.removeAttr('required');
      obj.parents('.form-group').find('span[style]').remove();
    });
  }
}


    
        SUMATERA BARAT SUMATERA SELATAN SUMATERA UTARA 3 results are available, use up and down arrow keys to navigate.SUMATERA BARATSUMATERA SELATANSUMATERA UTARASUMATERA UTARAKABUPATEN KARO SUMATERA UTARA1 result is available, use up and down arrow keys to navigate.KABUPATEN KARO
        
            
                 
            
        
        
        
        
            
                
                    
                        ×
                        Memuat... 
                    
                    
                        
                        
                        
                    
                
            
        

        
            
                
                    
                        ×
                        Memuat... 
                    
                    
                        
                        
                        
                        
                    
                
            
        

        
            
                
                    
                        ×
                        Memuat... 
                    
                    
                        
                        
                        
                    
                
            
        
        
            
                
                    
                        ×
                          Notifikasi
                    
                    
                        
                        
                    
                
            
        
                                
                 Webservice BPJS sedang Gangguan    
                X
            
                                        
	#modalBantuan {
        position: fixed;
        z-index: 99998; 
        height: 52px; 
        width: 52px;
        background-color: #25d366;
        text-align: center;
        bottom: 5%;
        left: 2%;
        border-radius: 50%;
        font-size: 40px;
    }

    #modalBantuan > i {
        color: white;
        padding-top: 5px;
    }






    function openBantuan(){
        Swal.fire({
            title: 'Apakah anda membutuhkan bantuan dari Team Customer Support?',
            type: 'info',
            showCancelButton: true,
            confirmButtonColor: '#3085d6',
            cancelButtonColor: '#d33',
            confirmButtonText: 'Ya',
            cancelButtonText: 'Tidak'
        }).then((result) => {
            if (result.value) {
                let notif_wa = &quot;https://api.whatsapp.com/send/?phone=6281277073300&amp;text=Hi,%20Saya%20pengguna%20ePuskesmas:%20,%20%20(darel)%20-%20P0000000001%20PUSKESMAS1%20-%20KABUPATEN%20BANDUNG%20memerlukan%20bantuan%20anda!&amp;app_absent=0&quot;
                window.open(notif_wa);
            }
        });
    }

    $(&quot;#modalBantuan&quot;).mousedown(function() {
        isDragging = false;
    }).mousemove(function() {
        isDragging = true;
    }).mouseup(function() {
        var wasDragging = isDragging;
        isDragging = false;
        if (!wasDragging) {
            openBantuan();
        }
    });

    dragElement(document.getElementById(&quot;modalBantuan&quot;));

    function dragElement(elmnt) {
      var pos1 = 0, pos2 = 0, pos3 = 0, pos4 = 0;
      if (document.getElementById(elmnt.id + &quot;header&quot;)) {
        document.getElementById(elmnt.id + &quot;header&quot;).onmousedown = dragMouseDown;
      } else {
        elmnt.onmousedown = dragMouseDown;
      }

      function dragMouseDown(e) {
        e = e || window.event;
        e.preventDefault();
        pos3 = e.clientX;
        pos4 = e.clientY;
        document.onmouseup = closeDragElement;
        document.onmousemove = elementDrag;
      }

      function elementDrag(e) {
        e = e || window.event;
        e.preventDefault();
        pos1 = pos3 - e.clientX;
        pos2 = pos4 - e.clientY;
        pos3 = e.clientX;
        pos4 = e.clientY;
        elmnt.style.top = (elmnt.offsetTop - pos2) + &quot;px&quot;;
        elmnt.style.left = (elmnt.offsetLeft - pos1) + &quot;px&quot;;
      }

      function closeDragElement() {
        document.onmouseup = null;
        document.onmousemove = null;
      }
    }

    var modalBantuanDrag = document.getElementById('modalBantuan');
    modalBantuanDrag.addEventListener('touchmove', function(event) {
      event.preventDefault();
      if (event.targetTouches.length == 1) {
        var touch = event.targetTouches[0];
        modalBantuanDrag.style.left = touch.pageX + 'px';
        modalBantuanDrag.style.top = touch.pageY + 'px';
      }
    }, {passive: true});
                
            $(document).ready(function(){
                $(window).scroll(function () {
                    $('#icon_back').attr('style','margin-top:'+(parseInt(innerHeight)-150)+'px;margin-left:'+(parseInt(innerWidth)-70)+'px;z-index:1000;');
                    if ($(this).scrollTop() > 125) {
                        $('#top-link-block').removeClass('hidden');
                        $('#top-link-block').fadeIn();
                    }else{
                        $('#top-link-block').fadeOut();
                    }                    
                });

                let message_skrining= 'null';
                if(message_skrining == &quot;error_message&quot;){

                    let text_message = 'null';
                    let text_status = 'null';
                    if(text_status == 'null'){
                        text_status = 'error';
                    }
                    if(text_message != 'null'){
                        Swal.fire(
                          text_message,
                          '',
                          text_status
                        )
                    }
                }

                $('[data-toggle=&quot;tooltip&quot;]').tooltip();


                                                                                        let stt= localStorage.getItem(&quot;count_notif&quot;);
                            if(stt !== &quot;undefined&quot; &amp;&amp; stt !== &quot;0&quot; &amp;&amp; stt !== &quot;null&quot; ){
                                $(&quot;#kolom_gagal_bridging&quot;).html(stt);   
                            }
                                                                            
            });
            
            function aksipesan(){
                Swal.fire({
                    text: &quot;Apakah Anda ingin tutup pesan Webservice bpjs dan tidak di tampilkan lagi ?&quot;,
                    type: 'warning',
                    showCancelButton: true,
                    confirmButtonColor: '#3085d6',
                    cancelButtonColor: '#d33',
                    confirmButtonText: 'Ya',
                    cancelButtonText: 'Tutup Sekarang Saja'
                }).then((result) => {
                    if(result.value){
                        setSessionWebSocket(1, &quot;indikatorbpjs&quot;);
                        $('#notif-connection-bpjs').hide();
                    }

                    if (result.dismiss == &quot;cancel&quot;) {
                        $('#notif-connection-bpjs').hide();
                    }
                });
            }

                        function callAntreanKiosk(nomorAntrean, ruangan_code){
                                return false;
            }
            
            function deletePasienKiosk(pasien_id){
                                return false;
            }

            function deletePendaftaranKiosk(pendaftaran_id){
                                return false;
            }

            function syncPasienKiosk(){
                                return false;
            }

            function syncPendaftaranKiosk(){
                                return false;
            }
        
        
            var date = new Date();
            var tanggal = new Date(date.getFullYear(), date.getMonth(), date.getDate(), 23, 59, 59, 59);
            var bulan = new Date(date.getFullYear(), date.getMonth(), 1, 0, 0, 0);
            var tahun = new Date(date.getFullYear(), 1, 0, 0, 0);
                            var tanggal_min = new Date(date.getFullYear(), date.getMonth(), date.getDate(), 00, 00, 00, 00);
                        var sistole_diastole_anamnesa = &quot;&quot;;
            var default_sistole = &quot;&quot;;
            var default_diastole = &quot;&quot;;
        
        
                    
            
                
            function showUpdateLog(from_menu = false)
            {
                $.ajax({
                    url: &quot;https://demo.epuskesmas.id/updatelog/showupdatelog&quot;,
                    method: 'GET',
                    dataType: &quot;json&quot;,
                    data: {
                        _token: &quot;1Qzy4frvPTBhCAOEI2lL6VeUfUPoVgVDUjUncrzY&quot;
                    },
                    success: function(data) {
                        if(data.title != '' &amp;&amp; data.title != undefined){
                            $('#modal .modal-title').html(data.title);
                            $('#modal .modal-form').html(data.form);
                            openmodal(&quot;modal-lg&quot;);
                        }else{
                            if(from_menu){
                                alert(&quot;Belum ada update terbaru saat ini!&quot;,&quot;info&quot;);
                            }
                        }
                    },
                    error: function (xhr, ajaxOptions, thrownError) {
                        alert(&quot;Terjadi kesalahan sistem, silahkan hubungi team support kami!&quot;, &quot;danger&quot;);
                        console.log(xhr.status);
                        console.log(thrownError);
                    }
                });
            }

            function showNotif(from_menu = false)
            {
                $.ajax({
                    url: &quot;https://demo.epuskesmas.id/mpuskesmas/shownotif&quot;,
                    method: 'GET',
                    dataType: &quot;json&quot;,
                    data: {
                        _token: &quot;1Qzy4frvPTBhCAOEI2lL6VeUfUPoVgVDUjUncrzY&quot;
                    },
                    success: function(data) {
                        if(data.title != '' &amp;&amp; data.title != undefined){
                            $('#modal_notif .modal-title').html(data.title);
                            $('#modal_notif .modal-form').html(data.form);
                            openmodal_notif(&quot;modal-lg&quot;);

                            if (data.notif=='NOTIF_02') {
                                $('#btn_close_suspend').hide();
                            }

                        }else{
                            if(from_menu){
                                alert(&quot;Belum ada update terbaru saat ini!&quot;,&quot;info&quot;);
                            }
                        }
                    },
                    error: function (xhr, ajaxOptions, thrownError) {
                        alert(&quot;Terjadi kesalahan sistem, silahkan hubungi team support kami!&quot;, &quot;danger&quot;);
                        console.log(xhr.status);
                        console.log(thrownError);
                    }
                });
            }

            function showNotifBridging(from_menu = false)
            {
                $.blockUI({
                css: {
                    border: 'none',
                    padding: '15px',
                    backgroundColor: '#000',
                    '-webkit-border-radius': '10px',
                    '-moz-border-radius': '10px',
                    opacity: .5,
                    fontSize: '8px',
                    color: '#fff'
                    },
                    message: '&lt;h3>Memuat....&lt;/h3>'
                });

                $.ajax({
                    url: &quot;https://demo.epuskesmas.id/home/shownotifbridging&quot;,
                    method: 'GET',
                    dataType: &quot;json&quot;,
                    data: {
                        _token: &quot;1Qzy4frvPTBhCAOEI2lL6VeUfUPoVgVDUjUncrzY&quot;
                    },
                    success: function(data) {
                        $('#modal_notif_bridging .modal-title').html(data.title);
                        $('#modal_notif_bridging .modal-form').html(data.form);
                        openmodal_notif_bridging(&quot;modal-lg&quot;);
                    },
                    error: function (xhr, ajaxOptions, thrownError) {
                        alert(&quot;Terjadi kesalahan sistem, silahkan hubungi team support kami!&quot;, &quot;danger&quot;);
                        console.log(xhr.status);
                        console.log(thrownError);
                    }
                });
            }

            function gagalBridgingCount(from_menu = false)
            {
                $.ajax({
                    url: &quot;https://demo.epuskesmas.id/home/gagalbridgingcount&quot;,
                    method: 'GET',
                    dataType: &quot;json&quot;,
                    data: {
                        _token: &quot;1Qzy4frvPTBhCAOEI2lL6VeUfUPoVgVDUjUncrzY&quot;
                    },
                    success: function(data) {
                        var x = document.getElementById(&quot;button_gagal_bridging&quot;);
                        if(data > 0){
                            $(&quot;#kolom_gagal_bridging&quot;).html(data);
                            localStorage.setItem('count_notif',data);    
                        }else{
                            localStorage.setItem('count_notif',&quot;0&quot;);
                        }
                    },
                    error: function (xhr, ajaxOptions, thrownError) {
                        alert(&quot;Terjadi kesalahan sistem, silahkan hubungi team support kami!&quot;, &quot;danger&quot;);
                        console.log(xhr.status);
                        console.log(thrownError);
                    }
                });
            }

        function showRiwayatPelayanan(obj)
        {
            data = $(obj).data();
            $.ajax({
                url: &quot;https://demo.epuskesmas.id/pelayanan/showriwayatpelayanan&quot;,
                method: &quot;GET&quot;,
                dataType: &quot;json&quot;,
                data: {
                id: data.id,
                puskesmas: data.puskesmas,
                _token: &quot;1Qzy4frvPTBhCAOEI2lL6VeUfUPoVgVDUjUncrzY&quot;,
            },
            success: function(data) {
                $(&quot;#modal .modal-title&quot;).html(data.title);
                $(&quot;#modal .modal-form&quot;).html(data.form);
                openmodal(&quot;modal-lg&quot;);
            },
            error: function (xhr, ajaxOptions, thrownError) {
                    alert(&quot;Terjadi kesalahan sistem, silahkan hubungi team support kami!&quot;, &quot;danger&quot;);
                    console.log(xhr.status);
                    console.log(thrownError);
                }
            });
        }

        function showRiwayatKunjunganBpjs(obj)
        {
            data = $(obj).data();
            $.ajax({
                url: &quot;https://demo.epuskesmas.id/pelayanan/showriwayatkunjunganbpjs&quot;,
                method: &quot;GET&quot;,
                dataType: &quot;json&quot;,
                data: {
                    id: data.id,
                    _token: &quot;1Qzy4frvPTBhCAOEI2lL6VeUfUPoVgVDUjUncrzY&quot;,
                },
                success: function(data) {
                    $(&quot;#modal .modal-title&quot;).html(data.title);
                    $(&quot;#modal .modal-form&quot;).html(data.form);
                    openmodal(&quot;modal-lg&quot;);
                },
                error: function (xhr, ajaxOptions, thrownError) {
                        alert(&quot;Terjadi kesalahan sistem, silahkan hubungi team support kami!&quot;, &quot;danger&quot;);
                        console.log(xhr.status);
                        console.log(thrownError);
                }
            });
        }

        function getRiwayatKunjunganBpjs(obj)
        {
            data = $(obj).data();
            $.ajax({
                url: &quot;https://demo.epuskesmas.id/pelayanan/getriwayatkunjunganbpjs&quot;,
                method: &quot;GET&quot;,
                dataType: &quot;json&quot;,
                data: {
                    noasuransi: data.noasuransi,
                    _token: &quot;1Qzy4frvPTBhCAOEI2lL6VeUfUPoVgVDUjUncrzY&quot;,
                },
                success: function(data) {
                    alert(data.message, data.status);
                    $(obj).removeClass('loading');
                    if(data.status == &quot;success&quot;){
                        window.location.reload(true);
                    }
                },
                error: function (xhr, ajaxOptions, thrownError) {
                    alert(&quot;Terjadi kesalahan sistem, silahkan hubungi team support kami!&quot;, &quot;danger&quot;);
                    console.log(xhr.status);
                    console.log(thrownError);
                }
            });
        }

        function setSessionWebSocket(action,to = null)
        {
            $.ajax({
                url: &quot;https://demo.epuskesmas.id/home/setsessionwebsocket&quot;,
                method: 'POST',
                dataType: &quot;json&quot;,
                data: {
                    action: action,
                    for_customsession:to,
                    _token: &quot;1Qzy4frvPTBhCAOEI2lL6VeUfUPoVgVDUjUncrzY&quot;
                },
                success: function(data) {
                    console.log(&quot;success&quot;);
                },
                error: function (xhr, ajaxOptions, thrownError) {
                    alert(&quot;Terjadi kesalahan sistem, silahkan hubungi team support kami!&quot;, &quot;danger&quot;);
                }
            });
        }

        
        

        
                        
                    $(`#checkEnvConfig`).html('&lt;center>Dashboard antrian realtime belum di setting&lt;/center>');
                    $(`#checkEnvConfigRanap`).html('&lt;center>Dashboard rawat inap realtime belum di setting&lt;/center>');
                
                        

id(&quot;form_creates&quot;)/div[@class=&quot;panel-body row&quot;]/div[@class=&quot;col-sm-4&quot;]/div[@class=&quot;panel panel-default container-fluid&quot;]/div[@class=&quot;panel-body row&quot;]/div[@class=&quot;form-group&quot;]/div[@class=&quot;col-sm-8 has-error&quot;]/input[@class=&quot;form-control input-sm uppercase ui-autocomplete-input&quot;]4 / 64</value>
      <webElementGuid>d199b6a5-238f-49d0-aca1-f87d799cd985</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>/html[1]/body[1]</value>
      <webElementGuid>d6cb4f22-cca4-4381-946c-6f2c1a441c20</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//body</value>
      <webElementGuid>6427399b-db29-43a5-9291-36d75f8d62f8</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//body[(text() = concat(&quot;
                            
	var cloud_host = &quot;&quot;;
									cloud_host = &quot; , &quot;'&quot; , &quot;http://10.10.10.10/home&quot; , &quot;'&quot; , &quot;;
				localStorage.setItem(&quot;epuskesmas-cloud-server&quot;, cloud_host);

	
	    #connection {
	        position: fixed;
	        font-weight: bold;
	        bottom: 5px;
	        left: 5px;
	        z-index: 99;
	        font-size: 16px;
	        border: none;
	        outline: none;
	        color: white;
	        cursor: pointer;
	        padding: 3px;
	        border-radius: 4px;
	    }
	    .online {
	      background: green;
	    }
	    .offline {
	      background: red;
	    }
	
	
	
	    window.onbeforeunload = function(event){
	    	var connection = document.getElementById(&quot; , &quot;'&quot; , &quot;connection&quot; , &quot;'&quot; , &quot;).innerHTML;
	    	localStorage.setItem(&quot; , &quot;'&quot; , &quot;epuskesmas-connection-status&quot; , &quot;'&quot; , &quot;, connection);
	    }
	    if (&quot; , &quot;'&quot; , &quot;serviceWorker&quot; , &quot;'&quot; , &quot; in navigator) {
	    	document.onreadystatechange = function(e){
	    		if(localStorage.getItem(&quot; , &quot;'&quot; , &quot;epuskesmas-connection-status&quot; , &quot;'&quot; , &quot;) != null &amp;&amp; localStorage.getItem(&quot; , &quot;'&quot; , &quot;epuskesmas-connection-status&quot; , &quot;'&quot; , &quot;).length > 0){
		            document.getElementById(&quot; , &quot;'&quot; , &quot;connection&quot; , &quot;'&quot; , &quot;).innerHTML = localStorage.getItem(&quot; , &quot;'&quot; , &quot;epuskesmas-connection-status&quot; , &quot;'&quot; , &quot;);
		            document.getElementById(&quot; , &quot;'&quot; , &quot;connection&quot; , &quot;'&quot; , &quot;).className = localStorage.getItem(&quot; , &quot;'&quot; , &quot;epuskesmas-connection-status&quot; , &quot;'&quot; , &quot;).toLowerCase();
	    		}
			}

	        window.addEventListener(&quot; , &quot;'&quot; , &quot;load&quot; , &quot;'&quot; , &quot;, function () {
	            navigator.serviceWorker.register(&quot; , &quot;'&quot; , &quot;/service-worker.js&quot; , &quot;'&quot; , &quot;).then(function(worker){
		    		if(localStorage.getItem(&quot; , &quot;'&quot; , &quot;epuskesmas-connection-status&quot; , &quot;'&quot; , &quot;) != null &amp;&amp; localStorage.getItem(&quot; , &quot;'&quot; , &quot;epuskesmas-connection-status&quot; , &quot;'&quot; , &quot;).length > 0){
		    			var loadConnection = localStorage.getItem(&quot; , &quot;'&quot; , &quot;epuskesmas-connection-status&quot; , &quot;'&quot; , &quot;).toLowerCase();
									                if(nextConnection == &quot; , &quot;'&quot; , &quot;online&quot; , &quot;'&quot; , &quot;){
			            		sweetAlert(&quot; , &quot;'&quot; , &quot;online&quot; , &quot;'&quot; , &quot;);
			                }else{
			            		sweetAlert(&quot; , &quot;'&quot; , &quot;offline&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;warning&quot; , &quot;'&quot; , &quot;);
			                }
								    			sweetAlert(localStorage.getItem(&quot; , &quot;'&quot; , &quot;epuskesmas-connection-status&quot; , &quot;'&quot; , &quot;).toLowerCase());
		    		}
	            });
	        });
	        
	        function send_message_to_service_worker(messages){
	            return new Promise(function(resolve, reject){
	                var messageChannel = new MessageChannel();
	                messageChannel.port1.onmessage = function(event){
	                    if(event.data.error){
	                        reject(event.data.error);
	                    }else{
	                        resolve(event.data);
	                    }
	                };
	                navigator.serviceWorker.controller.postMessage(messages, [messageChannel.port2]);
	            });
	        }

			navigator.serviceWorker.getRegistrations().then(registrations => {
				registrations.forEach(registration => {
					registration.unregister()
				})
			})

	        navigator.serviceWorker.addEventListener(&quot; , &quot;'&quot; , &quot;message&quot; , &quot;'&quot; , &quot;, function(event){
	        	if(event.data.type == &quot; , &quot;'&quot; , &quot;check connection&quot; , &quot;'&quot; , &quot;){
		            var previousConnection = document.getElementById(&quot; , &quot;'&quot; , &quot;connection&quot; , &quot;'&quot; , &quot;).className;
		            if(event.data.status != &quot;&quot;){
			            document.getElementById(&quot; , &quot;'&quot; , &quot;connection&quot; , &quot;'&quot; , &quot;).innerHTML = event.data.status;
			            document.getElementById(&quot; , &quot;'&quot; , &quot;connection&quot; , &quot;'&quot; , &quot;).className = event.data.status.toLowerCase();
		            }
		            var nextConnection = document.getElementById(&quot; , &quot;'&quot; , &quot;connection&quot; , &quot;'&quot; , &quot;).className;
								            if(previousConnection == &quot; , &quot;'&quot; , &quot;offline&quot; , &quot;'&quot; , &quot;){
			                if(nextConnection == &quot; , &quot;'&quot; , &quot;online&quot; , &quot;'&quot; , &quot;){
			            		sweetAlert(&quot; , &quot;'&quot; , &quot;online&quot; , &quot;'&quot; , &quot;);
			                }else{
			            		sweetAlert(&quot; , &quot;'&quot; , &quot;offline&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;warning&quot; , &quot;'&quot; , &quot;);
			                }
			            }else{
			                if(previousConnection == &quot; , &quot;'&quot; , &quot;online&quot; , &quot;'&quot; , &quot;){
			                	if(nextConnection == &quot; , &quot;'&quot; , &quot;offline&quot; , &quot;'&quot; , &quot;){
				            		sweetAlert(&quot; , &quot;'&quot; , &quot;offline&quot; , &quot;'&quot; , &quot;);
			                	}
			                }
			            }
						        	}
	        });

	        function sweetAlert(message = null, timer = 3000){
						        	switch(message) {
					  	case &quot; , &quot;'&quot; , &quot;offline&quot; , &quot;'&quot; , &quot;:
		                	Swal.fire({
		                       	html: &quot; , &quot;'&quot; , &quot;&lt;h5>ePuskesmas NG mendeteksi adanya gangguan pada koneksi internet.&lt;br>Pastikan perangkat anda terhubung dengan koneksi internet.&lt;h5>&quot; , &quot;'&quot; , &quot;,
								imageUrl: &quot; , &quot;'&quot; , &quot;https://demo.epuskesmas.id/images/o2o/offline.png&quot; , &quot;'&quot; , &quot;,
								imageWidth: 400,
								imageHeight: 175,
								allowOutsideClick: false,
								showConfirmButton: &quot;1&quot;,
		                       	confirmButtonColor: &quot; , &quot;'&quot; , &quot;#224d09&quot; , &quot;'&quot; , &quot;,
		                       	confirmButtonText: &quot; , &quot;'&quot; , &quot;Ya, Alihkan ke ePuskesmas NG Offline!&quot; , &quot;'&quot; , &quot;
		                   	}).then((result) => {
		                       	if (result.value) {
		                        	if(cloud_host == localStorage.getItem(&quot; , &quot;'&quot; , &quot;epuskesmas-cloud-server&quot; , &quot;'&quot; , &quot;)){
		                      			window.location = localStorage.getItem(&quot; , &quot;'&quot; , &quot;epuskesmas-cloud-server&quot; , &quot;'&quot; , &quot;);
		                        	}
		                       	}
		                   	})
					    	break;
		        		default:
			  				Swal.fire({
		                        html: &quot; , &quot;'&quot; , &quot;&lt;h5>ePuskesmas NG telah terhubung (online).&lt;h5>&quot; , &quot;'&quot; , &quot;,
								imageUrl: &quot; , &quot;'&quot; , &quot;https://demo.epuskesmas.id/images/o2o/online.png&quot; , &quot;'&quot; , &quot;,
								imageWidth: 400,
								imageHeight: 175,
		                        timer: timer,
								showConfirmButton: false
		                    });
					}
					        }

	        setInterval(function(){
	            send_message_to_service_worker({&quot; , &quot;'&quot; , &quot;type&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;check connection&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;url&quot; , &quot;'&quot; , &quot; :&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;});
	        }, 8000);
	    }
	
                
        
            
                
                
                    
                        Toggle navigation
                        
                    
                
                
                                                                   
                                                                                                
                                        Pendaftaran 
                                        
                                                                                        Pasien &amp; KK
                                                                                                                                    Pendaftaran Pasien
                                                                                                                                    Rekam Medis
                                                                                                                                    Antrean
                                                                                         
                                            Panggil Antrean
                                                                                    
                                    
                                
                                                                    
                                        Pelayanan 
                                        
                                                                                            Pelayanan
                                                                                                Medis
                                                                                                                                                Resep
                                                                                                                                                Obat Pasien
                                                                                                                                                Penimbangan Balita
                                                                                                                                                Diare Advokasi
                                                                                                                                                Program Puskesmas
                                                                                                                                        
                                                                                            Pelayanan Luar Gedung
                                                                                                    Skrining Anak Sekolah
                                                                                                                                                   Skrining Santri
                                                                                                                                                    Skrining PTM
                                                                                                                                                    Sinkronisasi Kunjungan Sehat BPJS
                                                                                                                                                    Sinkronisasi Data Sihepi
                                                                                            
                                                                                            Pemeriksaan
                                                                                                Laboratorium
                                                                                                                                            
                                                                                            Pasien Pulang
                                                                                                Rujukan External
                                                                                               
                                                                                                Pasien Meninggal
                                                                                                                                                Pembayaran
                                                                                                                                                Detail Transaksi
                                                                                            
                                                                                            Survey Kepuasan/Testimoni Pasien
                                                                                                Survey Kepuasan &amp; Testimoni Pasien
                                                                                                                                                Pengelolaan Survey Kepuasan &amp; Testimoni Pasien
                                                                                            
                                                                                            Lroa
                                                                                                Lroa
                                                                                                                                    
                                    
                                                            
                                                            
                                    Pengelolaan 
                                    
                                                                                    Gudang Farmasi
                                                                                        Pemesanan Obat
                                                                                                                                    Penerimaan Obat
                                                                                                                                    Stok Obat
                                                                                                                                    Stok Opname
                                                                                                                                    Distribusi Obat
                                                                                    
                                                                                    Data Master
                                                                                        Pegawai
                                                                                                                                    Pengguna
                                                                                                                                                                                Ruangan
                                                                                                                                    Ruangan Akses
                                                                                                                                    Ruangan Puskesmas
                                                                                                                                    Ruangan JAKESH
                                                                                                                                    Puskesmas
                                                                                                                                    Perlengkapan
                                                                                                                                    Sasaran Program
                                                                                                                                                                                            Sasaran Proyeksi
                                                                                                                                                                                    Jadwal Pelayanan
                                                                                                                                    Jadwal Shift
                                                                                                                                    Libur
                                                                                                                                    Data Tindakan
                                                                                                                                    Data Imunisasi
                                                                                                                                    Kamar &amp; Bed
                                                                                                                                    Sekolah
                                                                                                                                    Asuransi
                                                                                                                                    Laboratorium
                                                                                                                                    Paket Laboratorium
                                                                                                                                    Obat
                                                                                                                                    Konfigurasi Dashboard Antrian
                                                                                                                                    Konfigurasi Form
                                                                                                                                    Konfigurasi Laboratorium
                                                                                                                                    SMS Konfigurasi
                                                                                                                                    Konfigurasi Pembulatan
                                                                                                                                    Asupan Makanan
                                                                                                                                    Industri
                                                                                                                                    PBF
                                                                                                                                                                                Komponen Tarif
                                                                                                                                    Metode Pembayaran
                                                                                                                                                                                Diagnosa
                                                                                                                                    Template Print
                                                                                                                                                                      Data Dasar Puskesmas
                                        
                                                                                    BPJS
                                                                                        Pengaturan BPJS
                                                                                                                                    RS Provider BPJS
                                                                                                                                    Poli FKTL BPJS
                                                                                                                                    Dokter BPJS
                                                                                                                                    Tindakan BPJS
                                                                                                                                    Laboratorium BPJS
                                                                                                                                    Obat BPJS
                                                                                                                                    Ruangan BPJS
                                                                                                                                    Pengaturan Bridging BPJS
                                                                                                                                    Sinkron BPJS
                                                                                                                                    Sinkron BPJS Jejaring
                                                                                                                                
                                        

                                        
                                                                                    SISRUTE
                                                                                        Konfigurasi
                                                                                                                                    Tujuan Rujukan
                                                                                                                                    Kriteria Khusus
                                                                                       
                                                                                        SDM
                                                                                                                                    Ruang Perawatan
                                                                                                                                    Pelayanan
                                                                                                                                    Sarana
                                                                                                                                    Kelas Perawatan
                                                                                                                                    Alkes
                                                                                    
                                                                                    Master Wilayah
                                                                                        Wilayah Kerja
                                                                                                                                    Wilayah Kerja Posyandu
                                                                                    
                                                                                    SIPTM
                                                                                        Puskesmas
                                                                                                                                    Diagnosis
                                                                                                                                    Sinkron SIPTM
                                                                                    
                                        
                                                                                    Scan
                                                                                        Scan QRCode
                                                                                                                                                                        Antrian
                                                                                        Antrian
                                                                                                                                                                        Speaker
                                                                                        Speaker
                                                                                                                                                                
                                
                            
                                                            
                                    GIS 
                                    
                                                                                Pasien
                                                                                                                        Penyakit
                                                                            
                                
                                                                                        
                                    Laporan 
                                    
                                                                                    Grafik
                                            Grafik
                                                                                                                            Dashboard SIP
                                        
                                                                                    Monitoring
                                                                                                                                    Penggunaan Fitur
                                                                                    
                                                                                    Laporan Harian
                                                                                                                                    Pendapatan Tindakan
                                                                                                                                    Kunjungan Pasien
                                                                                                                                    Kunjungan Pasien BPJS
                                                                                                                                    Pelayanan Pasien
                                                                                                                                    Pemeriksaan Medis
                                                                                                                                    Pelayanan Resep
                                                                                                                                    Pengeluaran Obat Pasien
                                                                                                                                    Pengeluaran Alat Kesehatan
                                                                                                                                    Pelayanan Laboratorium
                                                                                                                                    Rujukan Internal
                                                                                                                                    Rujukan External
                                                                                                                                    
                                                                                                    Tindakan Dokter/Perawat
                                                                                            
                                                                                                                                    Pemeriksaan Laboratorium
                                                                                                                                    Kinerja Puskesmas
                                                                                                                                    Pelayanan Ruangan
                                                                                                                                    Penyakit
                                                                                                                                    Asuhan Keperawatan
                                                                                                                                    Kunjungan PTM
                                                                                                                                    PKPR
                                                                                                                                    Alat Kesehatan Terbanyak
                                                                                                                                    Tindakan Terbanyak
                                                                                                                                    Obat Kadaluarsa
                                                                                                                                    Laporan Pendapatan
                                                                                                                                    Laporan Pendapatan Pendaftaran 
                                             
                                                                                                                                                                                        Laporan Pendapatan Kasir
                                                                                                                                                                                    Laporan Transaksi Kasir
                                                                                                                                                                                Laporan Skrining COVID-19
                                                                                                                                    Laporan Pengeluaran Obat
                                                                                                                                    Laporan Stok Opname
                                                                                                                                        Laporan Jasa Tenaga Medis
                                                                                                                                        Laporan Hepatitis C
                                                                                                                                        Laporan Diare
                                                                                                                                        Laporan Hepatitis B
                                                                                                                                        Pendapatan Laboratorium
                                                                                    
                                                                                    Laporan Mingguan
                                                                                        Laporan PWS Penyakit
                                                                                    
                                                                                    Laporan Bulanan
                                                                                        Laporan
                                                                                                                                    Data Dasar
                                                                                                                                    Kirim Report
                                                                                                                                    Monitoring Laporan
                                                                                                                                    SP3 LB1
                                                                                                                                    SP3 LB2 (LPLPO)
                                                                                                                                    SP3 LB3
                                                                                                                                    SP3 LB4
                                                                                                                                    UKP-1. Pelayanan Puskesmas
                                                                                                                                    UKP-2. Kesakitan Umum
                                                                                                                                    UKP-3. Kesakitan Gigi dan Mulut
                                                                                                                                    UKP-4. Data Kesakitan Terbanyak
                                                                                                                                    UKP-5. Data Kematian di Puskesmas
                                                                                                                                    UKP-6. LPLPO
                                                                                                                                    UKME-1. Promosi Kesehatan
                                            UKME-2. Kesehatan Lingkungan
                                            UKME-3. Gizi, Kesehatan Ibu dan Kesehatan Anak
                                            UKME-4. Imunisasi
                                            UKME-5. Pengendalian Penyakit Menular
                                            UKME-6. Pengendalian Penyakit Tidak Menular
                                            UKME-7. Perawatan Kesehatan Masyarakat
                                            Penyakit Menular Potensi KLB
                                            Penyakit Potensi KLB Menurut Desa/Kel
                                            Promosi Kesehatan Data Kemitraan
                                            UKMP. Kesehatan Kerja
                                            Program
                                                                                                                                    Pemeriksaan IMS
                                                                                                                                    Laporan Kunjungan Ibu Hamil
                                                                                                                                    Kohort KIA
                                                                                                                                    PKPR
                                                                                                                                    Konseling HIV
                                                                                                                                    Rekapitulasi MTBS MTBM
                                                                                                                                    PTM
                                                                                                                                    Kirim Laporan
                                                                                        Laporan Tahunan
                                            Laporan Eliminasi 3E
                                                                                        Laporan Tahunan
                                            Laporan Tahunan LSD 1
                                                                                                                                    Laporan Tahunan LSD 2
                                                                                                                                    Laporan Tahunan LSD 3
                                                                                                                                                                                
                                                                            
                                
                            
                                                    
                                                               
                        
                            
	.broadcastBox::-webkit-scrollbar-track
    {
        -webkit-box-shadow: inset 0 0 6px rgba(0,0,0,0.3);
        background-color: #F5F5F5;
      border-radius: 5px
    }

    .broadcastBox::-webkit-scrollbar
    {
        width: 10px;
        background-color: #F5F5F5;
      border-radius: 5px
    }

    .broadcastBox::-webkit-scrollbar-thumb
    {
        background-color: black;
        border: 2px solid black;
      border-radius: 5px
    }

    .fasBroadcast{
      font-size: 25pt;
      padding-bottom: 10px;
      color: black;
      margin-right: 40px;
      margin-left: 40px;
    }

    .broadcastBox{
      width: 400px;
      height: 0px;
      border-radius: 10px;
      transition: 0.5s;
      position: absolute;
      overflow-y: scroll;
      padding: 0px;
      left: -360px;
      margin-top: 5px;
      background-color: #F4F4F4;
      -webkit-box-shadow: 10px 10px 23px 0px rgba(0,0,0,0.2);
      -moz-box-shadow: 10px 10px 23px 0px rgba(0,0,0,0.1);
      box-shadow: 10px 10px 23px 0px rgba(0,0,0,0.1);
    }

    .fasBroadcast:hover {
      color: #d63031;
    }


    .displayBroadcast{
      position: relative;
    }

    .contBroadcast{
      position: absolute;
      top: 0;
      width: 100%;
      height: 100%;
      background-color: #F4F4F4;
    }

    .contBroadcast:empty{
      display: none;
    }

    .stickBroadcast{
      text-align: center;  
      display: block;
      font-size: 50pt;
      padding-top: 70px;
      padding-left: 80px
    }

    .stickBroadcast:hover{
      color: black;
    }

    .centBroadcast{
      text-align: center;
      display: block;
    }

    .secBroadcast{
      padding: 25px 10px;
      background-color: #F4F4F4;
      transition: 0.5s;
    }

    .profContBroadcast{
      padding-left: 15px;
    }

    .profileBroadcast{
      -webkit-clip-path: circle(50% at 50% 50%);
      clip-path: circle(50% at 50% 50%);
      width: 75px;
      float: left;
    }

    .txtBroadcast{
      vertical-align: top;
      font-size: 1.25rem;
      padding: 5px 10px 0px 115px;
    }

    .subBroadcast{
      font-size: 1rem;
      color: grey;
    }

    .newBroadcast{
      border-style: none none solid none;
      border-color: red;
    }

    .secBroadcast:hover{
      background-color: #BFBFBF;
    }

    .headerBroadcast{
      padding: 5px;
    }

    .isiContentBroadcast{
      padding: 10px;
    }

    .judulBroadcast{
      padding-bottom: 10px;
    }

    .dotBroadcast {
      height: 9px;
      width: 9px;
      background-color: #9D2D15;
      border-radius: 50%;
      display: inline-block;
    }

    .containerNotifBroadcast img {
      transition: transform 0.25s ease;
      cursor: zoom-in;
    }

    .zoomCheckNotif:checked ~ label > img {
      transform: scale(2);
      cursor: zoom-out;
    }


  



    var openBroadcast = &quot;true&quot;
    let numClicks = 0;
    function showHideBroadcast() {
        if (openBroadcast == &quot;false&quot;) {
          $(&quot; , &quot;'&quot; , &quot;.broadcastBox&quot; , &quot;'&quot; , &quot;).removeAttr(&quot;style&quot;);
          openBroadcast = &quot;true&quot;;
          return false
        } 

        numClicks++;
        if (numClicks === 1) {
          singleClickTimer = setTimeout(() => {
            numClicks = 0;
          }, 400);
        } else if (numClicks === 2) {
          clearTimeout(singleClickTimer);
          numClicks = 0;
          return false
        }

        $.ajax({
            url: &quot;https://demo.epuskesmas.id/notifikasi/getform&quot;,
            method: &quot; , &quot;'&quot; , &quot;GET&quot; , &quot;'&quot; , &quot;,
            dataType: &quot;json&quot;,
            data: {
                _token: &quot;1Qzy4frvPTBhCAOEI2lL6VeUfUPoVgVDUjUncrzY&quot;
            },
            success: function(data) {
                $(&quot;#isiNotifBroadcast&quot;).html(data.data)
                setTimeout(function (){
                      if(openBroadcast == &quot;true&quot;) {
                          $(&quot; , &quot;'&quot; , &quot;.broadcastBox&quot; , &quot;'&quot; , &quot;).css({
                              height: &quot;60vh&quot;
                          });
                          openBroadcast = &quot;false&quot;;
                      }
                }, 100);
            },
            error: function (xhr, ajaxOptions, thrownError) {
                alert(&quot;Terjadi kesalahan sistem, silahkan hubungi team support kami!&quot;, &quot;danger&quot;);
                console.log(xhr.status);
                console.log(thrownError);
            }
        });

        
    }


    function setPilihanBroadcast() {
        var broadcastPilih = $(&quot; , &quot;'&quot; , &quot;select[name=&quot;broadcastPilihan&quot;]&quot; , &quot;'&quot; , &quot;).val()

        $.ajax({
            url: &quot;https://demo.epuskesmas.id/notifikasi/getform&quot;,
            method: &quot; , &quot;'&quot; , &quot;GET&quot; , &quot;'&quot; , &quot;,
            dataType: &quot;json&quot;,
            data: {
                pilihanBroadcast: broadcastPilih,
                _token: &quot;1Qzy4frvPTBhCAOEI2lL6VeUfUPoVgVDUjUncrzY&quot;
            },
            success: function(data) {
                $(&quot; , &quot;'&quot; , &quot;#contBroadcast&quot; , &quot;'&quot; , &quot;).html(&quot;&quot;)
                $(&quot; , &quot;'&quot; , &quot;#contBroadcast&quot; , &quot;'&quot; , &quot;).html(&quot; , &quot;'&quot; , &quot;&lt;br/>&lt;br/>&lt;span class=&quot;loading&quot; style=&quot;background-color:#F4F4F4!important;padding-top: 30px;padding-left: 90%;&quot;>&amp;nbsp;&lt;/span>&quot; , &quot;'&quot; , &quot;)
                setTimeout(function (){
                  $(&quot; , &quot;'&quot; , &quot;#contBroadcast&quot; , &quot;'&quot; , &quot;).html(data.data)
                }, 300);
            },
            error: function (xhr, ajaxOptions, thrownError) {
                alert(&quot;Terjadi kesalahan sistem, silahkan hubungi team support kami!&quot;, &quot;danger&quot;);
                console.log(xhr.status);
                console.log(thrownError);
            }
        }); 
    }

    function bacaSelengkapnyaBroadcast(ids) {
      $(&quot; , &quot;'&quot; , &quot;.broadcastBox&quot; , &quot;'&quot; , &quot;).removeAttr(&quot;style&quot;);
      openBroadcast = &quot;true&quot;;

      $.blockUI({
        css: {
            border: &quot; , &quot;'&quot; , &quot;none&quot; , &quot;'&quot; , &quot;,
            padding: &quot; , &quot;'&quot; , &quot;15px&quot; , &quot;'&quot; , &quot;,
            backgroundColor: &quot; , &quot;'&quot; , &quot;#000&quot; , &quot;'&quot; , &quot;,
            &quot; , &quot;'&quot; , &quot;-webkit-border-radius&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;10px&quot; , &quot;'&quot; , &quot;,
            &quot; , &quot;'&quot; , &quot;-moz-border-radius&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;10px&quot; , &quot;'&quot; , &quot;,
            opacity: .5,
            fontSize: &quot; , &quot;'&quot; , &quot;8px&quot; , &quot;'&quot; , &quot;,
            color: &quot; , &quot;'&quot; , &quot;#fff&quot; , &quot;'&quot; , &quot;
            },
            message: &quot; , &quot;'&quot; , &quot;&lt;h3>Memuat....&lt;/h3>&quot; , &quot;'&quot; , &quot;
        });

      $.unblockUI()

      $.ajax({
          url: &quot;https://demo.epuskesmas.id/notifikasi/show&quot;,
          method: &quot; , &quot;'&quot; , &quot;GET&quot; , &quot;'&quot; , &quot;,
          dataType: &quot;json&quot;,
          data: {
              ids: ids,
              _token: &quot;1Qzy4frvPTBhCAOEI2lL6VeUfUPoVgVDUjUncrzY&quot;
          },
          success: function(data) {
              $(&quot;#modal_notif_broadcast_notifikasi&quot;).modal({ backdrop: &quot; , &quot;'&quot; , &quot;static&quot; , &quot;'&quot; , &quot;});
              $(&quot;#modal_notif_broadcast_notifikasi .modal-dialog&quot;).addClass(&quot;modal-lg&quot;);
              $(&quot; , &quot;'&quot; , &quot;#modal_notif_broadcast_notifikasi .modal-title&quot; , &quot;'&quot; , &quot;).html(data.tipe);
              $(&quot; , &quot;'&quot; , &quot;#modal_notif_broadcast_notifikasi .modal-form&quot; , &quot;'&quot; , &quot;).html(data.data);
              console.log(data.ressRead)
              if (data.ressRead == false) {
                $(&quot;#dotBroadcast&quot;).addClass(&quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;)
              }
          },
          error: function (xhr, ajaxOptions, thrownError) {
              alert(&quot;Terjadi kesalahan sistem, silahkan hubungi team support kami!&quot;, &quot;danger&quot;);
              console.log(xhr.status);
              console.log(thrownError);
          }
      });   
    }

    function skemaAction() {
      let skema = $(&quot;#skema&quot;).val()
      $(&quot;#skemaAction&quot;).addClass(&quot; , &quot;'&quot; , &quot;loading&quot; , &quot;'&quot; , &quot;)
      var broadcastPilih = $(&quot; , &quot;'&quot; , &quot;select[name=&quot;broadcastPilihan&quot;]&quot; , &quot;'&quot; , &quot;).val()
      $.ajax({
          url: &quot;https://demo.epuskesmas.id/notifikasi/getformother&quot;,
          method: &quot; , &quot;'&quot; , &quot;GET&quot; , &quot;'&quot; , &quot;,
          dataType: &quot;json&quot;,
          data: {
              skema: skema,
              pilihanBroadcast: broadcastPilih,
              _token: &quot;1Qzy4frvPTBhCAOEI2lL6VeUfUPoVgVDUjUncrzY&quot;
          },
          success: function(data) {
            setTimeout(function (){
            $(&quot;#skemaAction&quot;).removeClass(&quot; , &quot;'&quot; , &quot;loading&quot; , &quot;'&quot; , &quot;)
            let totalSkema = Number(skema) + 1
              $(&quot;#skema&quot;).val(totalSkema)
              $(&quot;#isiDetailNotif&quot;).append(data.data)

              if(data.nextAction != &quot;true&quot;) {
                $(&quot;#skemaAction&quot;).addClass(&quot;hidden&quot;)
              }
            }, 300);
          },
          error: function (xhr, ajaxOptions, thrownError) {
              $(&quot;#skemaAction&quot;).removeClass(&quot; , &quot;'&quot; , &quot;loading&quot; , &quot;'&quot; , &quot;)
              alert(&quot;Terjadi kesalahan sistem, silahkan hubungi team support kami!&quot;, &quot;danger&quot;);
              console.log(xhr.status);
              console.log(thrownError);
          }
      }); 
    }
                        
                        
                                                             
                                                                        
                                         Belum Bridging
                                        8      
                                    
                                                                                                                        
                                                    
                                
                                    Darel PUSKESMAS PUSKESMAS1 
                                
                                
                                    Profil
                                                                            Update
                                                                        Umpan Balik
                                                                            Bantuan
                                                                        
                                    
                                        
                                            Keluar
                                        

                                        
                                            
                                        
                                    
                                
                            
                                            
                
            
        

        
        
        
        

        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        

        
                    backgroundChart = &quot;images/gis/bg-grafik.png&quot;;
                
        
        

        
        
            

    
        Buat Baru Pasien
        
                                            Simpan
                Pendaftaran
                        
            Lihat Semua
            Reset
        

    
    
        
        
        
            
                              
                    Data Pasien
                    
                                            
                            
                            Diverifikasi (lengkap)
                        
                    
                
                                              
                    Data PKM Lain
                    
                        
                        
                    
                
                                
                    No. eRM *
                    
                        
                                                            - Otomatis -
                                
                                                    
                    
                
                                
                    Penjamin
                    
                        
                                                                                                Umum

                                                                    BPJS Kesehatan

                                                                    Pemerintah Daerah Kota

                                                                    Bpjs ketenagakerjaan

                                                                    BPJS KESEHATAN RUJUK INTERNAL

                                                                    ASURANSI U/ HIDUP

                                                                    Umum Baru

                                                                    asuransi

                                                                    UMUM

                                                                                    
                    
                
                
                    No. Penjamin 
                    
                        
                            
                             
                        

                                                
                            
                        
                    
                
                
                    No. KK 
                    
                        
                    
                
                                
                    NIK  *
                    
                        
                            
                                                    
                    
                
                
                    
                  
                                      
                
                
                
                    Nama *
                    
                        
                    
                
                
                    Jenis Kelamin *
                    
                        
                            
                                
                                Laki-laki
                            
                            
                                
                                Perempuan
                            
                        
                    
                
                
                    Tanggal Lahir  *
                    
                        
                            
                            
                        
                    
                
                
                    Umur 
                    
                        
                            
                            Thn
                            
                            Bln
                            
                            Hr
                        
                    
                
                
                    Tempat Lahir  *
                    
                        
                    
                
                                            

            
                                
                    No. Dokumen RM 
                    
                        
                    
                
                                                
                    No. RM Lama 
                    
                        
                    
                
                                                
                    Golongan Darah 
                    
                        
                            - Pilih -
                                                                                                A
                                                                    B
                                                                    AB
                                                                    O
                                                                    A-
                                                                    A+
                                                                    B-
                                                                    B+
                                                                    AB+
                                                                    AB-
                                                                    O+
                                                                    O-
                                                                                    
                    
                
                
                    E-mail
                    
                        
                    
                
                
                    No. HP  *
                    
                        
                    
                
                                                
                    
                        Alamat Tempat Tinggal / KTP sama 
                    
                    
                                        
                            Propinsi  *
                            
                                
                                
                            
                        
                        
                            Kota/Kab  *
                            
                                
                                
                            
                        
                        
                            Kecamatan  *
                            
                                
                                
                            
                        
                        
                            Kelurahan/Desa  *
                            
                                
                                
                            
                        
                        
                            Dusun 
                            
                                
                                
                            
                        
                        
                            Alamat  *
                            
                                
                            
                        
                        
                            RT/RW   *
                            
                                
                                    
                                    
                                
                            
                        
                                        
                
                                        
            

            
                                
                    
                        
                            
                            Alamat KTP Jika Berbeda
                    
                    
                        
                            
                                Propinsi 
                                
                                    
                                    
                                
                            
                            
                                Kota/Kab 
                                
                                    
                                    
                                
                            
                            
                                Kecamatan 
                                
                                    
                                    
                                
                            
                            
                                Kelurahan/Desa 
                                
                                    
                                    
                                
                            
                            
                                Dusun 
                                
                                    
                                    
                                
                            
                            
                                Alamat 
                                
                                    
                                
                            
                            
                                RT/RW 
                                
                                    
                                        
                                        
                                    
                                
                            
                        

                    
                
                                
                    Pekerjaan  *
                    
                        
                        
                    
                
                                
                    Pekerjaan Suami
                    
                        
                        
                    
                
                
                    Instansi 
                    
                        
                    
                
                                
                    Agama  *
                    
                        
                            - Pilih -
                                                                                                ISLAM
                                                                    KATOLIK
                                                                    KRISTEN
                                                                    HINDU
                                                                    BUDDHA
                                                                    KONGHUCU
                                                                    LAINNYA
                                                                                    
                    
                
                
                    Pendidikan  *
                    
                        
                            - Pilih -
                                                                                                TIDAK/BELUM SEKOLAH
                                                                    BELUM TAMAT SD/SEDERAJAT
                                                                    TAMAT SD/SEDERAJAT
                                                                    SLTP/SEDERAJAT
                                                                    SLTA/SEDERAJAT
                                                                    DIPLOMA I
                                                                    DIPLOMA II
                                                                    AKADEMI/DIPLOMA III/SARJANA MUDA
                                                                    DIPLOMA IV/STRATA I
                                                                    STRATA II
                                                                    STRATA III
                                                                                    
                    
                
                
                    Status Perkawinan  *
                    
                        
                            - Pilih -
                                                                                                BELUM KAWIN
                                                                    KAWIN
                                                                    CERAI HIDUP
                                                                    CERAI MATI
                                                                                    
                    
                
                                
                    Tanggal Menikah
                    
                        
                            
                        
                        
                    
                
                
                    Status Keluarga  *
                    
                        
                            - Pilih -
                                                                                                KEPALA KELUARGA
                                                                    SUAMI
                                                                    ISTRI
                                                                    ANAK
                                                                    MENANTU
                                                                    CUCU
                                                                    ORANG TUA
                                                                    MERTUA
                                                                    PEMBANTU
                                                                    LAINNYA
                                                                                    
                    
                
                                                                
                    Warga Negara  *
                    
                        
                            - Pilih -
                                                                                                INDONESIA
                                                                    ASING
                                                                                    
                    
                
                
                    No. Paspor 
                    
                        
                            
                            
                        
                    
                
                
                    Nama Ayah/KK 
                    
                        
                    
                
                
                    Nama Ibu 
                    
                        
                    
                
                            

        
        
                            Simpan
                 Cetak
                 Kartu Pasien
                        
            
            
            Reset
        
    



history.pushState(null, null, location.href);
window.onpopstate = function () {
    history.go(1);
};
$(&quot;.date&quot;).on(&quot;dp.change&quot;, function() {
  var getdata = generateUmur();
  $(&quot; , &quot;'&quot; , &quot;#umur_tahun&quot; , &quot;'&quot; , &quot;).val(getdata[&quot; , &quot;'&quot; , &quot;years&quot; , &quot;'&quot; , &quot;]);
  $(&quot; , &quot;'&quot; , &quot;#umur_bulan&quot; , &quot;'&quot; , &quot;).val(getdata[&quot; , &quot;'&quot; , &quot;months&quot; , &quot;'&quot; , &quot;]);
  $(&quot; , &quot;'&quot; , &quot;#umur_hari&quot; , &quot;'&quot; , &quot;).val(getdata[&quot; , &quot;'&quot; , &quot;days&quot; , &quot;'&quot; , &quot;]);
});
$(&quot; , &quot;'&quot; , &quot;#tanggal_lahir&quot; , &quot;'&quot; , &quot;).keyup(function(){
  if($(&quot; , &quot;'&quot; , &quot;#tanggal_lahir&quot; , &quot;'&quot; , &quot;).val().length == 10){
    var getdata = generateUmur();
    $(&quot; , &quot;'&quot; , &quot;#umur_tahun&quot; , &quot;'&quot; , &quot;).val(getdata[&quot; , &quot;'&quot; , &quot;years&quot; , &quot;'&quot; , &quot;]);
    $(&quot; , &quot;'&quot; , &quot;#umur_bulan&quot; , &quot;'&quot; , &quot;).val(getdata[&quot; , &quot;'&quot; , &quot;months&quot; , &quot;'&quot; , &quot;]);
    $(&quot; , &quot;'&quot; , &quot;#umur_hari&quot; , &quot;'&quot; , &quot;).val(getdata[&quot; , &quot;'&quot; , &quot;days&quot; , &quot;'&quot; , &quot;]);
  }
});
$(&quot; , &quot;'&quot; , &quot;#umur_tahun&quot; , &quot;'&quot; , &quot;).keyup(function(){
  generateUmurToBirthday();  
});
$(&quot; , &quot;'&quot; , &quot;#umur_bulan&quot; , &quot;'&quot; , &quot;).keyup(function(){
  generateUmurToBirthday();
});
$(&quot; , &quot;'&quot; , &quot;#umur_hari&quot; , &quot;'&quot; , &quot;).keyup(function(){
  generateUmurToBirthday();
});

function generateUmurToBirthday()
{
  var bDay = moment().subtract($(&quot; , &quot;'&quot; , &quot;#umur_tahun&quot; , &quot;'&quot; , &quot;).val(), &quot; , &quot;'&quot; , &quot;years&quot; , &quot;'&quot; , &quot;);
  bDay.subtract($(&quot; , &quot;'&quot; , &quot;#umur_bulan&quot; , &quot;'&quot; , &quot;).val(), &quot; , &quot;'&quot; , &quot;months&quot; , &quot;'&quot; , &quot;);
  bDay.subtract($(&quot; , &quot;'&quot; , &quot;#umur_hari&quot; , &quot;'&quot; , &quot;).val(), &quot; , &quot;'&quot; , &quot;days&quot; , &quot;'&quot; , &quot;);
  bDay = bDay.format(&quot;DD-MM-YYYY&quot;);
  $(&quot; , &quot;'&quot; , &quot;#tanggal_lahir&quot; , &quot;'&quot; , &quot;).val(bDay);
}
$(&quot;input[name=&quot; , &quot;'&quot; , &quot;cari_pasien&quot; , &quot;'&quot; , &quot;&quot;).autocomplete({
    delay: 1500,
    source: function( request, response ) {
        $.ajax({
            url: &quot;https://demo.epuskesmas.id/pasien/create&quot;,
            method: &quot;GET&quot;,
            dataType: &quot;json&quot;,
            data: {
                autocomplete: &quot; , &quot;'&quot; , &quot;pasien&quot; , &quot;'&quot; , &quot;,
                search: {&quot; , &quot;'&quot; , &quot;cari_pasien&quot; , &quot;'&quot; , &quot;:request.term}
            },
            success: function(data) {
                response(data);
            },
            error: function (xhr, ajaxOptions, thrownError) {
                console.log(xhr.status);
                console.log(thrownError);
            }
        });
    },
    focus: function(event,ui){
        $(this).parent().removeClass(&quot; , &quot;'&quot; , &quot;has-success&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;has-error&quot; , &quot;'&quot; , &quot;);
        return false;
    },
    select: function(event,ui){
        $(this).parent().removeClass(&quot; , &quot;'&quot; , &quot;has-error&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;has-success&quot; , &quot;'&quot; , &quot;);
        getPasienData(ui.item);     
    },
    minLength: 5
});


$(&quot; , &quot;'&quot; , &quot;.date&quot; , &quot;'&quot; , &quot;).datetimepicker({
      format:&quot; , &quot;'&quot; , &quot;DD-MM-YYYY&quot; , &quot;'&quot; , &quot;,
      useCurrent: false,
  });

function getPasienData(item)
{
    $.ajax({
        url: &quot;https://demo.epuskesmas.id/pasien/create&quot;,
        method: &quot;GET&quot;,
        dataType: &quot;json&quot;,
        data: {
            autocomplete: &quot; , &quot;'&quot; , &quot;get_pasien&quot; , &quot;'&quot; , &quot;,
            search: {&quot; , &quot;'&quot; , &quot;get_pasien&quot; , &quot;'&quot; , &quot;:item.id}
        },
        success: function(data) {
            if(data.length > 0) {
                data.forEach(function(items,index){
                    setDataDariKota(items);
                });
                umur = generateUmur()
                $(&quot; , &quot;'&quot; , &quot;#umur_tahun&quot; , &quot;'&quot; , &quot;).val(umur[&quot; , &quot;'&quot; , &quot;years&quot; , &quot;'&quot; , &quot;]);
                $(&quot; , &quot;'&quot; , &quot;#umur_bulan&quot; , &quot;'&quot; , &quot;).val(umur[&quot; , &quot;'&quot; , &quot;months&quot; , &quot;'&quot; , &quot;]);
                $(&quot; , &quot;'&quot; , &quot;#umur_hari&quot; , &quot;'&quot; , &quot;).val(umur[&quot; , &quot;'&quot; , &quot;days&quot; , &quot;'&quot; , &quot;]);
            }
        },
        error: function (xhr, ajaxOptions, thrownError) {
            console.log(xhr.status);
            console.log(thrownError);
        }
    });
}

function generateUmur()
{
  var arr = [];
  var tes = $(&quot; , &quot;'&quot; , &quot;#tanggal_lahir&quot; , &quot;'&quot; , &quot;).val();
  var real = moment(&quot;&quot;+tes+&quot;&quot;, &quot;DD-MM-YYYY&quot;);
  var now = moment();
  var years = now.diff(real, &quot; , &quot;'&quot; , &quot;years&quot; , &quot;'&quot; , &quot;);
  real.add(years, &quot; , &quot;'&quot; , &quot;years&quot; , &quot;'&quot; , &quot;);

  var months = now.diff(real, &quot; , &quot;'&quot; , &quot;months&quot; , &quot;'&quot; , &quot;);
  real.add(months, &quot; , &quot;'&quot; , &quot;months&quot; , &quot;'&quot; , &quot;);

  var days = now.diff(real, &quot; , &quot;'&quot; , &quot;days&quot; , &quot;'&quot; , &quot;);

  arr[&quot; , &quot;'&quot; , &quot;years&quot; , &quot;'&quot; , &quot;] = years;
  arr[&quot; , &quot;'&quot; , &quot;months&quot; , &quot;'&quot; , &quot;] = months;
  arr[&quot; , &quot;'&quot; , &quot;days&quot; , &quot;'&quot; , &quot;] = days;
  
  return arr;
}

function generateUmurByDate(data)
{
  var arr = [];
  var birthdate = moment(&quot;&quot;+data+&quot;&quot;, &quot;DD-MM-YYYY&quot;);
  var now = moment();

  var years = now.diff(birthdate, &quot; , &quot;'&quot; , &quot;year&quot; , &quot;'&quot; , &quot;);
  birthdate.add(years, &quot; , &quot;'&quot; , &quot;years&quot; , &quot;'&quot; , &quot;);

  var months = now.diff(birthdate, &quot; , &quot;'&quot; , &quot;months&quot; , &quot;'&quot; , &quot;);
  birthdate.add(months, &quot; , &quot;'&quot; , &quot;months&quot; , &quot;'&quot; , &quot;);

  var days = now.diff(birthdate, &quot; , &quot;'&quot; , &quot;days&quot; , &quot;'&quot; , &quot;);
  arr[&quot; , &quot;'&quot; , &quot;years&quot; , &quot;'&quot; , &quot;] = years;
  arr[&quot; , &quot;'&quot; , &quot;months&quot; , &quot;'&quot; , &quot;] = months;
  arr[&quot; , &quot;'&quot; , &quot;days&quot; , &quot;'&quot; , &quot;] = days;
  
  return arr;
}
$(&quot;input[name=&quot; , &quot;'&quot; , &quot;MPasien[no_kk]&quot; , &quot;'&quot; , &quot;&quot;).autocomplete({
    delay: 1000,
    source: function( request, response ) {
        $.ajax({
            url: &quot;https://demo.epuskesmas.id/pasien/create&quot;,
            method: &quot;GET&quot;,
            dataType: &quot;json&quot;,
            data: {
                autocomplete: &quot; , &quot;'&quot; , &quot;no_kk&quot; , &quot;'&quot; , &quot;,
                search: {&quot; , &quot;'&quot; , &quot;no_kk&quot; , &quot;'&quot; , &quot;:request.term}
            },
            success: function(data) {
                response(data);
            },
            error: function (xhr, ajaxOptions, thrownError) {
                console.log(xhr.status);
                console.log(thrownError);
            }
        });
    },
    focus: function(event,ui){
        $(this).parent().removeClass(&quot; , &quot;'&quot; , &quot;has-success&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;has-error&quot; , &quot;'&quot; , &quot;);
        return false;
    },
    select: function(event,ui){
        $(this).parent().removeClass(&quot; , &quot;'&quot; , &quot;has-error&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;has-success&quot; , &quot;'&quot; , &quot;);
        setDataDariKK(ui.item);
    },
    minLength: 5
});

function setDataDariKota(item)
{
    setDataDariKK(item);
    $(&quot; , &quot;'&quot; , &quot;#created_id&quot; , &quot;'&quot; , &quot;).html(item.id);

    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[verified_by]&quot;]&quot; , &quot;'&quot; , &quot;).prop(&quot;checked&quot;, false);
    if(item.verified_by != &quot;&quot; &amp;&amp; item.verified_by != null){
        $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[nik]&quot;]&quot; , &quot;'&quot; , &quot;).attr(&quot;required&quot;, false);
        $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[verified_by]&quot;]&quot; , &quot;'&quot; , &quot;).prop(&quot;checked&quot;, true);
    }else{
        $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[nik]&quot;]&quot; , &quot;'&quot; , &quot;).attr(&quot;required&quot;, true);
        $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[verified_by]&quot;]&quot; , &quot;'&quot; , &quot;).prop(&quot;checked&quot;, false);
    }
    setRequiredForm(false);

    if(item.ktp_alamat_berbeda == 1 || item.ktp_alamat_berbeda == true){
        $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[ktp_propinsi_id]&quot;]&quot; , &quot;'&quot; , &quot;).attr(&quot;required&quot;, false);
        if($(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[ktp_alamat_berbeda]&quot;]&quot; , &quot;'&quot; , &quot;).is(&quot; , &quot;'&quot; , &quot;:not(:checked)&quot; , &quot;'&quot; , &quot;)){
            $(&quot;#collapseAlamatKtp&quot;).collapse(&quot;show&quot;);
            $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[ktp_alamat_berbeda]&quot;]&quot; , &quot;'&quot; , &quot;).prop(&quot;checked&quot;, true);
        }
    }else{
        $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[ktp_propinsi_id]&quot;]&quot; , &quot;'&quot; , &quot;).attr(&quot;required&quot;, true);
        $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[ktp_alamat_berbeda]&quot;]&quot; , &quot;'&quot; , &quot;).prop(&quot;checked&quot;, false);
        $(&quot; , &quot;'&quot; , &quot;#collapseAlamatKtp&quot; , &quot;'&quot; , &quot;).collapse(&quot;hide&quot;);
    }
    setRequiredFormKtp(false);

    $(&quot; , &quot;'&quot; , &quot;select[name=&quot;MPasien[asuransi_id]&quot;]&quot; , &quot;'&quot; , &quot;).val(item.asuransi_id).change();
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[no_asuransi]&quot;]&quot; , &quot;'&quot; , &quot;).val(item.no_asuransi);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[id]&quot;]&quot; , &quot;'&quot; , &quot;).val(item.id);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[nama]&quot;]&quot; , &quot;'&quot; , &quot;).val(item.nama);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[nik]&quot;]&quot; , &quot;'&quot; , &quot;).val(item.nik);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[no_rm_lama]&quot;]&quot; , &quot;'&quot; , &quot;).val(item.no_rm_lama);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[jenis_kelamin]&quot;][value=&quot;&quot; , &quot;'&quot; , &quot;+item.jenis_kelamin+&quot; , &quot;'&quot; , &quot;&quot;]&quot; , &quot;'&quot; , &quot;).prop(&quot;checked&quot;, true);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[tempat_lahir]&quot;]&quot; , &quot;'&quot; , &quot;).val(item.tempat_lahir);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[tanggal_lahir]&quot;]&quot; , &quot;'&quot; , &quot;).val(item.tanggal_lahir);
    $(&quot; , &quot;'&quot; , &quot;select[name=&quot;MPasien[gol_darah]&quot;]&quot; , &quot;'&quot; , &quot;).val(item.gol_darah);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[no_hp]&quot;]&quot; , &quot;'&quot; , &quot;).val(item.no_hp);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[pekerjaan_id]&quot;]&quot; , &quot;'&quot; , &quot;).val(item.pekerjaan_id);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[instansi]&quot;]&quot; , &quot;'&quot; , &quot;).val(item.instansi);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;pekerjaan_nama&quot;]&quot; , &quot;'&quot; , &quot;).val(item.pekerjaan_nama);
    $(&quot; , &quot;'&quot; , &quot;select[name=&quot;MPasien[agama]&quot;]&quot; , &quot;'&quot; , &quot;).val(item.agama);
    $(&quot; , &quot;'&quot; , &quot;select[name=&quot;MPasien[pendidikan]&quot;]&quot; , &quot;'&quot; , &quot;).val(item.pendidikan);
    $(&quot; , &quot;'&quot; , &quot;select[name=&quot;MPasien[status_perkawinan]&quot;]&quot; , &quot;'&quot; , &quot;).val(item.status_perkawinan);
    $(&quot; , &quot;'&quot; , &quot;select[name=&quot;MPasien[status_keluarga]&quot;]&quot; , &quot;'&quot; , &quot;).val(item.status_keluarga);
    $(&quot; , &quot;'&quot; , &quot;select[name=&quot;MPasien[warganegara]&quot;]&quot; , &quot;'&quot; , &quot;).val(item.warganegara);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[nama_ibu]&quot;]&quot; , &quot;'&quot; , &quot;).val(item.nama_ibu);

    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[ktp_propinsi_id]&quot;]&quot; , &quot;'&quot; , &quot;).val(item.ktp_propinsi_id);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;ktp_propinsi_nama&quot;]&quot; , &quot;'&quot; , &quot;).val(item.ktp_propinsi_nama);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[ktp_kota_id]&quot;]&quot; , &quot;'&quot; , &quot;).val(item.ktp_kota_id);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;ktp_kota_nama&quot;]&quot; , &quot;'&quot; , &quot;).val(item.ktp_kota_nama);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[ktp_kecamatan_id]&quot;]&quot; , &quot;'&quot; , &quot;).val(item.ktp_kecamatan_id);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;ktp_kecamatan_nama&quot;]&quot; , &quot;'&quot; , &quot;).val(item.ktp_kecamatan_nama);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[ktp_kelurahan_id]&quot;]&quot; , &quot;'&quot; , &quot;).val(item.ktp_kelurahan_id);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;ktp_kelurahan_nama&quot;]&quot; , &quot;'&quot; , &quot;).val(item.ktp_kelurahan_nama);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[ktp_dusun_id]&quot;]&quot; , &quot;'&quot; , &quot;).val(item.ktp_dusun_id);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;ktp_dusun_nama&quot;]&quot; , &quot;'&quot; , &quot;).val(item.ktp_dusun_nama);
    $(&quot; , &quot;'&quot; , &quot;textarea[name=&quot;MPasien[ktp_alamat]&quot;]&quot; , &quot;'&quot; , &quot;).val(item.ktp_alamat);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[ktp_rt]&quot;]&quot; , &quot;'&quot; , &quot;).val(item.ktp_rt);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[ktp_rw]&quot;]&quot; , &quot;'&quot; , &quot;).val(item.ktp_rw);

    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[imd]&quot;]&quot; , &quot;'&quot; , &quot;).prop(&quot;checked&quot;, false);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[bb_lahir]&quot;]&quot; , &quot;'&quot; , &quot;).val(item.bb_lahir);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[tb_lahir]&quot;]&quot; , &quot;'&quot; , &quot;).val(item.tb_lahir);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[imd]&quot;][value=&quot;&quot; , &quot;'&quot; , &quot;+ item.imd +&quot; , &quot;'&quot; , &quot;&quot;]&quot; , &quot;'&quot; , &quot;).prop(&quot; , &quot;'&quot; , &quot;checked&quot; , &quot;'&quot; , &quot;, true);

}

function setDataDariKK(item)
{
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[propinsi_id]&quot;]&quot; , &quot;'&quot; , &quot;).val(item.propinsi_id);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;propinsi_nama&quot;]&quot; , &quot;'&quot; , &quot;).val(item.propinsi_nama);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[kota_id]&quot;]&quot; , &quot;'&quot; , &quot;).val(item.kota_id);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;kota_nama&quot;]&quot; , &quot;'&quot; , &quot;).val(item.kota_nama);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[kecamatan_id]&quot;]&quot; , &quot;'&quot; , &quot;).val(item.kecamatan_id);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;kecamatan_nama&quot;]&quot; , &quot;'&quot; , &quot;).val(item.kecamatan_nama);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[kelurahan_id]&quot;]&quot; , &quot;'&quot; , &quot;).val(item.kelurahan_id);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;kelurahan_nama&quot;]&quot; , &quot;'&quot; , &quot;).val(item.kelurahan_nama);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[dusun_id]&quot;]&quot; , &quot;'&quot; , &quot;).val(item.dusun_id);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;dusun_nama&quot;]&quot; , &quot;'&quot; , &quot;).val(item.dusun_nama);
    $(&quot; , &quot;'&quot; , &quot;textarea[name=&quot;MPasien[alamat]&quot;]&quot; , &quot;'&quot; , &quot;).val(item.alamat);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[rt]&quot;]&quot; , &quot;'&quot; , &quot;).val(item.rt);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[rw]&quot;]&quot; , &quot;'&quot; , &quot;).val(item.rw);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[nama_ayah]&quot;]&quot; , &quot;'&quot; , &quot;).val(item.nama_ayah);
}

$(&quot;input[name=&quot; , &quot;'&quot; , &quot;pekerjaan_nama&quot; , &quot;'&quot; , &quot;&quot;).autocomplete({
    delay: 1000,
    source: function( request, response ) {
        $.ajax({
            url: &quot;https://demo.epuskesmas.id/pasien/create&quot;,
            method: &quot;GET&quot;,
            dataType: &quot;json&quot;,
            data: {
                autocomplete: &quot; , &quot;'&quot; , &quot;pekerjaan&quot; , &quot;'&quot; , &quot;,
                search: {&quot; , &quot;'&quot; , &quot;pekerjaan_nama&quot; , &quot;'&quot; , &quot;:request.term}
            },
            success: function(data) {
                response(data);
            },
            error: function (xhr, ajaxOptions, thrownError) {
                console.log(xhr.status);
                console.log(thrownError);
            }
        });
    },
    focus: function(event,ui){
        $(this).parent().removeClass(&quot; , &quot;'&quot; , &quot;has-success&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;has-error&quot; , &quot;'&quot; , &quot;);
        return false;
    },
    select: function(event,ui){
        $(this).parent().removeClass(&quot; , &quot;'&quot; , &quot;has-error&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;has-success&quot; , &quot;'&quot; , &quot;);
        $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[pekerjaan_id]&quot;]&quot; , &quot;'&quot; , &quot;).val(ui.item.id);
    },
    minLength: 3
});
$(&quot;input[name=&quot; , &quot;'&quot; , &quot;pekerjaan_suami_nama&quot; , &quot;'&quot; , &quot;&quot;).autocomplete({
    delay: 1000,
    source: function( request, response ) {
        $.ajax({
            url: &quot;https://demo.epuskesmas.id/pasien/create&quot;,
            method: &quot;GET&quot;,
            dataType: &quot;json&quot;,
            data: {
                autocomplete: &quot; , &quot;'&quot; , &quot;pekerjaan&quot; , &quot;'&quot; , &quot;,
                search: {&quot; , &quot;'&quot; , &quot;pekerjaan_nama&quot; , &quot;'&quot; , &quot;:request.term}
            },
            success: function(data) {
                response(data);
            },
            error: function (xhr, ajaxOptions, thrownError) {
                console.log(xhr.status);
                console.log(thrownError);
            }
        });
    },
    focus: function(event,ui){
        $(this).parent().removeClass(&quot; , &quot;'&quot; , &quot;has-success&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;has-error&quot; , &quot;'&quot; , &quot;);
        return false;
    },
    select: function(event,ui){
        $(this).parent().removeClass(&quot; , &quot;'&quot; , &quot;has-error&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;has-success&quot; , &quot;'&quot; , &quot;);
        $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[pekerjaan_suami_id]&quot;]&quot; , &quot;'&quot; , &quot;).val(ui.item.id);
    },
    minLength: 3
});
$(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[instansi]&quot;]&quot; , &quot;'&quot; , &quot;).autocomplete({
    delay: 1000,
    source: function( request, response ) {
        $.ajax({
            url: &quot;https://demo.epuskesmas.id/pasien/create&quot;,
            method: &quot;GET&quot;,
            dataType: &quot;json&quot;,
            data: {
                autocomplete: &quot; , &quot;'&quot; , &quot;instansi&quot; , &quot;'&quot; , &quot;,
                search: {&quot; , &quot;'&quot; , &quot;instansi&quot; , &quot;'&quot; , &quot;:request.term}
            },
            success: function(data) {
                response(data);
            },
            error: function (xhr, ajaxOptions, thrownError) {
                console.log(xhr.status);
                console.log(thrownError);
            }
        });
    },
    focus: function(event,ui){
        return false;
    },
    select: function(event,ui){
        $(this).parent().removeClass(&quot; , &quot;'&quot; , &quot;has-error&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;has-success&quot; , &quot;'&quot; , &quot;);
    },
    minLength: 3
});

function setWilayah(item)
{
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[propinsi_id]&quot;]&quot; , &quot;'&quot; , &quot;).val(item.propinsi_id);
    $(&quot; , &quot;'&quot; , &quot;input[name=propinsi_nama]&quot; , &quot;'&quot; , &quot;).val(item.propinsi_nama);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[kota_id]&quot;]&quot; , &quot;'&quot; , &quot;).val(item.kota_id);
    $(&quot; , &quot;'&quot; , &quot;input[name=kota_nama]&quot; , &quot;'&quot; , &quot;).val(item.kota_nama);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[kecamatan_id]&quot;]&quot; , &quot;'&quot; , &quot;).val(item.kecamatan_id);
    $(&quot; , &quot;'&quot; , &quot;input[name=kecamatan_nama]&quot; , &quot;'&quot; , &quot;).val(item.kecamatan_nama);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[kelurahan_id]&quot;]&quot; , &quot;'&quot; , &quot;).val(item.kelurahan_id);
    $(&quot; , &quot;'&quot; , &quot;input[name=kelurahan_nama]&quot; , &quot;'&quot; , &quot;).val(item.kelurahan_nama);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[dusun_id]&quot;]&quot; , &quot;'&quot; , &quot;).val(item.dusun_id);
    $(&quot; , &quot;'&quot; , &quot;input[name=dusun_nama]&quot; , &quot;'&quot; , &quot;).val(item.dusun_nama);
}

function setWilayahKtp(item)
{
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[ktp_propinsi_id]&quot;]&quot; , &quot;'&quot; , &quot;).val(item.propinsi_id);
    $(&quot; , &quot;'&quot; , &quot;input[name=ktp_propinsi_nama]&quot; , &quot;'&quot; , &quot;).val(item.propinsi_nama);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[ktp_kota_id]&quot;]&quot; , &quot;'&quot; , &quot;).val(item.kota_id);
    $(&quot; , &quot;'&quot; , &quot;input[name=ktp_kota_nama]&quot; , &quot;'&quot; , &quot;).val(item.kota_nama);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[ktp_kecamatan_id]&quot;]&quot; , &quot;'&quot; , &quot;).val(item.kecamatan_id);
    $(&quot; , &quot;'&quot; , &quot;input[name=ktp_kecamatan_nama]&quot; , &quot;'&quot; , &quot;).val(item.kecamatan_nama);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[ktp_kelurahan_id]&quot;]&quot; , &quot;'&quot; , &quot;).val(item.kelurahan_id);
    $(&quot; , &quot;'&quot; , &quot;input[name=ktp_kelurahan_nama]&quot; , &quot;'&quot; , &quot;).val(item.kelurahan_nama);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[ktp_dusun_id]&quot;]&quot; , &quot;'&quot; , &quot;).val(item.dusun_id);
    $(&quot; , &quot;'&quot; , &quot;input[name=ktp_dusun_nama]&quot; , &quot;'&quot; , &quot;).val(item.dusun_nama);
}

$(&quot;input[name=&quot; , &quot;'&quot; , &quot;propinsi_nama&quot; , &quot;'&quot; , &quot;&quot;).autocomplete({
    delay: 1000,
    source: function( request, response ) {
        $.ajax({
            url: &quot;https://demo.epuskesmas.id/pasien/create&quot;,
            method: &quot;GET&quot;,
            dataType: &quot;json&quot;,
            data: {
                autocomplete: &quot; , &quot;'&quot; , &quot;propinsi&quot; , &quot;'&quot; , &quot;,
                search: {&quot; , &quot;'&quot; , &quot;propinsi_nama&quot; , &quot;'&quot; , &quot;:request.term}
            },
            success: function(data) {
                response(data);
            },
            error: function (xhr, ajaxOptions, thrownError) {
                console.log(xhr.status);
                console.log(thrownError);
            }
        });
    },
    focus: function(event,ui){
        $(this).parent().removeClass(&quot; , &quot;'&quot; , &quot;has-success&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;has-error&quot; , &quot;'&quot; , &quot;);
        return false;
    },
    select: function(event,ui){
        setWilayah(ui.item);
        $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[propinsi_id]&quot;]&quot; , &quot;'&quot; , &quot;).val(ui.item.id);
        $(this).parent().removeClass(&quot; , &quot;'&quot; , &quot;has-error&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;has-success&quot; , &quot;'&quot; , &quot;);
    },
    minLength: 3
});

$(&quot;input[name=&quot; , &quot;'&quot; , &quot;kota_nama&quot; , &quot;'&quot; , &quot;]&quot;).autocomplete({
    delay: 1000,
    source: function( request, response ) {
        $.ajax({
            url: &quot;https://demo.epuskesmas.id/pasien/create&quot;,
            method: &quot;GET&quot;,
            dataType: &quot;json&quot;,
            data: {
                autocomplete: &quot; , &quot;'&quot; , &quot;kota&quot; , &quot;'&quot; , &quot;,
                search: {
                    &quot; , &quot;'&quot; , &quot;propinsi_id&quot; , &quot;'&quot; , &quot;:$(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[propinsi_id]&quot;]&quot; , &quot;'&quot; , &quot;).val(),
                    &quot; , &quot;'&quot; , &quot;kota_nama&quot; , &quot;'&quot; , &quot;:request.term
                }
            },
            success: function(data) {
                response(data);
            },
            error: function (xhr, ajaxOptions, thrownError) {
                console.log(xhr.status);
                console.log(thrownError);
            }
        });
    },
    focus: function(event,ui){
        $(this).parent().removeClass(&quot; , &quot;'&quot; , &quot;has-success&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;has-error&quot; , &quot;'&quot; , &quot;);
        return false;
    },
    select: function(event,ui){
        setWilayah(ui.item);
        $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[kota_id]&quot;]&quot; , &quot;'&quot; , &quot;).val(ui.item.id);
        $(this).parent().removeClass(&quot; , &quot;'&quot; , &quot;has-error&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;has-success&quot; , &quot;'&quot; , &quot;);
    },
    minLength: 3
});

$(&quot;input[name=&quot; , &quot;'&quot; , &quot;kecamatan_nama&quot; , &quot;'&quot; , &quot;]&quot;).autocomplete({
    delay: 1000,
    source: function( request, response ) {
        $.ajax({
            url: &quot;https://demo.epuskesmas.id/pasien/create&quot;,
            method: &quot;GET&quot;,
            dataType: &quot;json&quot;,
            data: {
                autocomplete: &quot; , &quot;'&quot; , &quot;kecamatan&quot; , &quot;'&quot; , &quot;,
                search: {
                    &quot; , &quot;'&quot; , &quot;kota_id&quot; , &quot;'&quot; , &quot;:$(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[kota_id]&quot;]&quot; , &quot;'&quot; , &quot;).val(),
                    &quot; , &quot;'&quot; , &quot;kecamatan_nama&quot; , &quot;'&quot; , &quot;:request.term
                }
            },
            success: function(data) {
                response(data);
            },
            error: function (xhr, ajaxOptions, thrownError) {
                console.log(xhr.status);
                console.log(thrownError);
            }
        });
    },
    focus: function(event,ui){
        $(this).parent().removeClass(&quot; , &quot;'&quot; , &quot;has-success&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;has-error&quot; , &quot;'&quot; , &quot;);
        return false;
    },
    select: function(event,ui){
        setWilayah(ui.item);
        $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[kecamatan_id]&quot;]&quot; , &quot;'&quot; , &quot;).val(ui.item.id);
        $(this).parent().removeClass(&quot; , &quot;'&quot; , &quot;has-error&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;has-success&quot; , &quot;'&quot; , &quot;);
    },
    minLength: 3
});

$(&quot;input[name=&quot; , &quot;'&quot; , &quot;kelurahan_nama&quot; , &quot;'&quot; , &quot;]&quot;).autocomplete({
    delay: 1000,
    source: function( request, response ) {
        $.ajax({
            url: &quot;https://demo.epuskesmas.id/pasien/create&quot;,
            method: &quot;GET&quot;,
            dataType: &quot;json&quot;,
            data: {
                autocomplete: &quot; , &quot;'&quot; , &quot;kelurahan&quot; , &quot;'&quot; , &quot;,
                search: {
                    &quot; , &quot;'&quot; , &quot;kecamatan_id&quot; , &quot;'&quot; , &quot;:$(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[kecamatan_id]&quot;]&quot; , &quot;'&quot; , &quot;).val(),
                    &quot; , &quot;'&quot; , &quot;kelurahan_nama&quot; , &quot;'&quot; , &quot;:request.term
                }
            },
            success: function(data) {
                response(data);
            },
            error: function (xhr, ajaxOptions, thrownError) {
                console.log(xhr.status);
                console.log(thrownError);
            }
        });
    },
    focus: function(event,ui){
        $(this).parent().removeClass(&quot; , &quot;'&quot; , &quot;has-success&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;has-error&quot; , &quot;'&quot; , &quot;);
        return false;
    },
    select: function(event,ui){
        setWilayah(ui.item);
        $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[kelurahan_id]&quot;]&quot; , &quot;'&quot; , &quot;).val(ui.item.id);
        $(this).parent().removeClass(&quot; , &quot;'&quot; , &quot;has-error&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;has-success&quot; , &quot;'&quot; , &quot;);
    },
    minLength: 3
});

$(&quot;input[name=&quot; , &quot;'&quot; , &quot;dusun_nama&quot; , &quot;'&quot; , &quot;]&quot;).autocomplete({
    delay: 1000,
    source: function( request, response ) {
        $.ajax({
            url: &quot;https://demo.epuskesmas.id/pasien/create&quot;,
            method: &quot;GET&quot;,
            dataType: &quot;json&quot;,
            data: {
                autocomplete: &quot; , &quot;'&quot; , &quot;dusun&quot; , &quot;'&quot; , &quot;,
                search: {
                    &quot; , &quot;'&quot; , &quot;kelurahan_id&quot; , &quot;'&quot; , &quot;:$(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[kelurahan_id]&quot;]&quot; , &quot;'&quot; , &quot;).val(),
                    &quot; , &quot;'&quot; , &quot;dusun_nama&quot; , &quot;'&quot; , &quot;:request.term
                }
            },
            success: function(data) {
                response(data);
            },
            error: function (xhr, ajaxOptions, thrownError) {
                console.log(xhr.status);
                console.log(thrownError);
            }
        });
    },
    focus: function(event,ui){
        $(this).parent().removeClass(&quot; , &quot;'&quot; , &quot;has-success&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;has-error&quot; , &quot;'&quot; , &quot;);
        return false;
    },
    select: function(event,ui){
        setWilayah(ui.item);
        $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[dusun_id]&quot;]&quot; , &quot;'&quot; , &quot;).val(ui.item.id);
        $(this).parent().removeClass(&quot; , &quot;'&quot; , &quot;has-error&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;has-success&quot; , &quot;'&quot; , &quot;);
    },
    minLength: 3
});

$(&quot;input[name=&quot; , &quot;'&quot; , &quot;ktp_propinsi_nama&quot; , &quot;'&quot; , &quot;&quot;).autocomplete({
    delay: 1000,
    source: function( request, response ) {
        $.ajax({
            url: &quot;https://demo.epuskesmas.id/pasien/create&quot;,
            method: &quot;GET&quot;,
            dataType: &quot;json&quot;,
            data: {
                autocomplete: &quot; , &quot;'&quot; , &quot;propinsi&quot; , &quot;'&quot; , &quot;,
                search: {&quot; , &quot;'&quot; , &quot;propinsi_nama&quot; , &quot;'&quot; , &quot;:request.term}
            },
            success: function(data) {
                response(data);
            },
            error: function (xhr, ajaxOptions, thrownError) {
                console.log(xhr.status);
                console.log(thrownError);
            }
        });
    },
    focus: function(event,ui){
        $(this).parent().removeClass(&quot; , &quot;'&quot; , &quot;has-success&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;has-error&quot; , &quot;'&quot; , &quot;);
        return false;
    },
    select: function(event,ui){
        setWilayahKtp(ui.item);
        $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[ktp_propinsi_id]&quot;]&quot; , &quot;'&quot; , &quot;).val(ui.item.id);
        $(this).parent().removeClass(&quot; , &quot;'&quot; , &quot;has-error&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;has-success&quot; , &quot;'&quot; , &quot;);
    },
    minLength: 3
});

$(&quot;input[name=&quot; , &quot;'&quot; , &quot;ktp_kota_nama&quot; , &quot;'&quot; , &quot;]&quot;).autocomplete({
    delay: 1000,
    source: function( request, response ) {
        $.ajax({
            url: &quot;https://demo.epuskesmas.id/pasien/create&quot;,
            method: &quot;GET&quot;,
            dataType: &quot;json&quot;,
            data: {
                autocomplete: &quot; , &quot;'&quot; , &quot;kota&quot; , &quot;'&quot; , &quot;,
                search: {
                    &quot; , &quot;'&quot; , &quot;propinsi_id&quot; , &quot;'&quot; , &quot;:$(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[ktp_propinsi_id]&quot;]&quot; , &quot;'&quot; , &quot;).val(),
                    &quot; , &quot;'&quot; , &quot;kota_nama&quot; , &quot;'&quot; , &quot;:request.term
                }
            },
            success: function(data) {
                response(data);
            },
            error: function (xhr, ajaxOptions, thrownError) {
                console.log(xhr.status);
                console.log(thrownError);
            }
        });
    },
    focus: function(event,ui){
        $(this).parent().removeClass(&quot; , &quot;'&quot; , &quot;has-success&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;has-error&quot; , &quot;'&quot; , &quot;);
        return false;
    },
    select: function(event,ui){
        setWilayahKtp(ui.item);
        $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[ktp_kota_id]&quot;]&quot; , &quot;'&quot; , &quot;).val(ui.item.id);
        $(this).parent().removeClass(&quot; , &quot;'&quot; , &quot;has-error&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;has-success&quot; , &quot;'&quot; , &quot;);
    },
    minLength: 3
});

$(&quot;input[name=&quot; , &quot;'&quot; , &quot;ktp_kecamatan_nama&quot; , &quot;'&quot; , &quot;]&quot;).autocomplete({
    delay: 1000,
    source: function( request, response ) {
        $.ajax({
            url: &quot;https://demo.epuskesmas.id/pasien/create&quot;,
            method: &quot;GET&quot;,
            dataType: &quot;json&quot;,
            data: {
                autocomplete: &quot; , &quot;'&quot; , &quot;kecamatan&quot; , &quot;'&quot; , &quot;,
                search: {
                    &quot; , &quot;'&quot; , &quot;kota_id&quot; , &quot;'&quot; , &quot;:$(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[ktp_kota_id]&quot;]&quot; , &quot;'&quot; , &quot;).val(),
                    &quot; , &quot;'&quot; , &quot;kecamatan_nama&quot; , &quot;'&quot; , &quot;:request.term
                }
            },
            success: function(data) {
                response(data);
            },
            error: function (xhr, ajaxOptions, thrownError) {
                console.log(xhr.status);
                console.log(thrownError);
            }
        });
    },
    focus: function(event,ui){
        $(this).parent().removeClass(&quot; , &quot;'&quot; , &quot;has-success&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;has-error&quot; , &quot;'&quot; , &quot;);
        return false;
    },
    select: function(event,ui){
        setWilayahKtp(ui.item);
        $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[ktp_kecamatan_id]&quot;]&quot; , &quot;'&quot; , &quot;).val(ui.item.id);
        $(this).parent().removeClass(&quot; , &quot;'&quot; , &quot;has-error&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;has-success&quot; , &quot;'&quot; , &quot;);
    },
    minLength: 3
});

$(&quot;input[name=&quot; , &quot;'&quot; , &quot;ktp_kelurahan_nama&quot; , &quot;'&quot; , &quot;]&quot;).autocomplete({
    delay: 1000,
    source: function( request, response ) {
        $.ajax({
            url: &quot;https://demo.epuskesmas.id/pasien/create&quot;,
            method: &quot;GET&quot;,
            dataType: &quot;json&quot;,
            data: {
                autocomplete: &quot; , &quot;'&quot; , &quot;kelurahan&quot; , &quot;'&quot; , &quot;,
                search: {
                    &quot; , &quot;'&quot; , &quot;kecamatan_id&quot; , &quot;'&quot; , &quot;:$(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[ktp_kecamatan_id]&quot;]&quot; , &quot;'&quot; , &quot;).val(),
                    &quot; , &quot;'&quot; , &quot;kelurahan_nama&quot; , &quot;'&quot; , &quot;:request.term
                }
            },
            success: function(data) {
                response(data);
            },
            error: function (xhr, ajaxOptions, thrownError) {
                console.log(xhr.status);
                console.log(thrownError);
            }
        });
    },
    focus: function(event,ui){
        $(this).parent().removeClass(&quot; , &quot;'&quot; , &quot;has-success&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;has-error&quot; , &quot;'&quot; , &quot;);
        return false;
    },
    select: function(event,ui){
        setWilayahKtp(ui.item);
        $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[ktp_kelurahan_id]&quot;]&quot; , &quot;'&quot; , &quot;).val(ui.item.id);
        $(this).parent().removeClass(&quot; , &quot;'&quot; , &quot;has-error&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;has-success&quot; , &quot;'&quot; , &quot;);
    },
    minLength: 3
});

$(&quot;input[name=&quot; , &quot;'&quot; , &quot;ktp_dusun_nama&quot; , &quot;'&quot; , &quot;]&quot;).autocomplete({
    delay: 1000,
    source: function( request, response ) {
        $.ajax({
            url: &quot;https://demo.epuskesmas.id/pasien/create&quot;,
            method: &quot;GET&quot;,
            dataType: &quot;json&quot;,
            data: {
                autocomplete: &quot; , &quot;'&quot; , &quot;dusun&quot; , &quot;'&quot; , &quot;,
                search: {
                    &quot; , &quot;'&quot; , &quot;kelurahan_id&quot; , &quot;'&quot; , &quot;:$(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[ktp_kelurahan_id]&quot;]&quot; , &quot;'&quot; , &quot;).val(),
                    &quot; , &quot;'&quot; , &quot;dusun_nama&quot; , &quot;'&quot; , &quot;:request.term
                }
            },
            success: function(data) {
                response(data);
            },
            error: function (xhr, ajaxOptions, thrownError) {
                console.log(xhr.status);
                console.log(thrownError);
            }
        });
    },
    focus: function(event,ui){
        $(this).parent().removeClass(&quot; , &quot;'&quot; , &quot;has-success&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;has-error&quot; , &quot;'&quot; , &quot;);
        return false;
    },
    select: function(event,ui){
        setWilayahKtp(ui.item);
        $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[ktp_dusun_id]&quot;]&quot; , &quot;'&quot; , &quot;).val(ui.item.id);
        $(this).parent().removeClass(&quot; , &quot;'&quot; , &quot;has-error&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;has-success&quot; , &quot;'&quot; , &quot;);
    },
    minLength: 3
});

function setDataDariApiBpjs(peserta)
{
    var style = peserta.ketAktif == &quot;AKTIF&quot; ? &quot;text-success&quot; : &quot;text-danger&quot;;
    var status = &quot;text-danger&quot;;
    if(peserta.status == &quot; , &quot;'&quot; , &quot;Sesuai Faskes&quot; , &quot;'&quot; , &quot;){
        status = &quot;text-success&quot;;
    }else if(peserta.status == &quot; , &quot;'&quot; , &quot;Faskes Jejaring&quot; , &quot;'&quot; , &quot;){
        status = &quot;text-info&quot;;
    }
    html = &quot; , &quot;'&quot; , &quot;Status: &lt;b>&lt;span class=&quot; , &quot;'&quot; , &quot;+style+&quot; , &quot;'&quot; , &quot;>&quot; , &quot;'&quot; , &quot;+peserta.ketAktif+&quot; , &quot;'&quot; , &quot;&lt;\/span>&lt;\/b>&lt;br>&quot; , &quot;'&quot; , &quot;
            +&quot; , &quot;'&quot; , &quot;Kode Provider: &quot; , &quot;'&quot; , &quot;+peserta.kdProviderPst.kdProvider+&quot; , &quot;'&quot; , &quot;&lt;br>&quot; , &quot;'&quot; , &quot;
            +&quot; , &quot;'&quot; , &quot;Nama Provider: &quot; , &quot;'&quot; , &quot;+peserta.kdProviderPst.nmProvider+&quot; , &quot;'&quot; , &quot;&lt;br>&quot; , &quot;'&quot; , &quot;
            +&quot; , &quot;'&quot; , &quot;Nama Peserta: &quot; , &quot;'&quot; , &quot;+(peserta.nama).toUpperCase()+&quot; , &quot;'&quot; , &quot;&lt;br>&quot; , &quot;'&quot; , &quot;
            +&quot; , &quot;'&quot; , &quot;Kelas : &quot; , &quot;'&quot; , &quot;+peserta.jnsKelas.nama+&quot; , &quot;'&quot; , &quot;&lt;br>&quot; , &quot;'&quot; , &quot;
            +&quot; , &quot;'&quot; , &quot;Jenis Peserta : &quot; , &quot;'&quot; , &quot;+peserta.jnsPeserta.nama+&quot; , &quot;'&quot; , &quot;&lt;br>&quot; , &quot;'&quot; , &quot;
            +&quot; , &quot;'&quot; , &quot;Prolanis/PRB : &quot; , &quot;'&quot; , &quot;+(peserta.pstProl != &quot;&quot; ? &quot; , &quot;'&quot; , &quot;Ya(&quot; , &quot;'&quot; , &quot;+peserta.pstProl+&quot; , &quot;'&quot; , &quot;)&quot; , &quot;'&quot; , &quot; : &quot;Bukan&quot;)+(peserta.pstPrb != &quot;&quot; ? &quot; , &quot;'&quot; , &quot;/Ya(&quot; , &quot;'&quot; , &quot;+peserta.pstPrb+&quot; , &quot;'&quot; , &quot;)&quot; , &quot;'&quot; , &quot; : &quot;/Bukan&quot;)+&quot; , &quot;'&quot; , &quot;&lt;br>&quot; , &quot;'&quot; , &quot;
            +&quot; , &quot;'&quot; , &quot;Tunggakan: &quot; , &quot;'&quot; , &quot;+peserta.tunggakan+&quot; , &quot;'&quot; , &quot;&lt;br>&quot; , &quot;'&quot; , &quot;
            +&quot; , &quot;'&quot; , &quot;&lt;b>&lt;span class=&quot; , &quot;'&quot; , &quot;+status+&quot; , &quot;'&quot; , &quot;>&quot; , &quot;'&quot; , &quot;+peserta.status+&quot; , &quot;'&quot; , &quot;&lt;\/span>&lt;\/b>&lt;br>&quot; , &quot;'&quot; , &quot;;
    $(&quot; , &quot;'&quot; , &quot;#data_peserta_bpjs&quot; , &quot;'&quot; , &quot;).html(html);


    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[no_asuransi]&quot;]&quot; , &quot;'&quot; , &quot;).val(peserta.noKartu);
    if(peserta.noKTP != null){
        if(peserta.noKTP.trim() != &quot;&quot;){
            $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[nik]&quot;]&quot; , &quot;'&quot; , &quot;).val(peserta.noKTP);
        }
    }
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[nama]&quot;]&quot; , &quot;'&quot; , &quot;).val(peserta.nama);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[jenis_kelamin]&quot;][value=&quot;&quot; , &quot;'&quot; , &quot;+peserta.sex+&quot; , &quot;'&quot; , &quot;&quot;]&quot; , &quot;'&quot; , &quot;).attr(&quot;checked&quot;,true);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[tanggal_lahir]&quot;]&quot; , &quot;'&quot; , &quot;).val(peserta.tglLahir);
    $(&quot; , &quot;'&quot; , &quot;select[name=&quot;MPasien[gol_darah]&quot;]&quot; , &quot;'&quot; , &quot;).val(peserta.golDarah);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[no_hp]&quot;]&quot; , &quot;'&quot; , &quot;).val(peserta.noHP);
    $(&quot; , &quot;'&quot; , &quot;select[name=&quot;MPasien[status_keluarga]&quot;]&quot; , &quot;'&quot; , &quot;).val(peserta.hubunganKeluarga.toUpperCase());
    if(peserta.tglLahir != &quot;&quot;){
      var tgl = moment(&quot;&quot;+peserta.tglLahir+&quot;&quot;, &quot;DD-MM-YYYY&quot;).format(&quot;DD-MM-YYYY&quot;);
      var date = generateUmurByDate(tgl);
      $(&quot; , &quot;'&quot; , &quot;#umur_tahun&quot; , &quot;'&quot; , &quot;).val(date[&quot; , &quot;'&quot; , &quot;years&quot; , &quot;'&quot; , &quot;]);
      $(&quot; , &quot;'&quot; , &quot;#umur_bulan&quot; , &quot;'&quot; , &quot;).val(date[&quot; , &quot;'&quot; , &quot;months&quot; , &quot;'&quot; , &quot;]);
      $(&quot; , &quot;'&quot; , &quot;#umur_hari&quot; , &quot;'&quot; , &quot;).val(date[&quot; , &quot;'&quot; , &quot;days&quot; , &quot;'&quot; , &quot;]);
    }
}

function setAsuransi()
{
    var asuransi_id = $(&quot; , &quot;'&quot; , &quot;select[name=&quot;MPasien[asuransi_id]&quot;]&quot; , &quot;'&quot; , &quot;);
    var is_bridgingbpjs = asuransi_id.find(&quot; , &quot;'&quot; , &quot;:selected&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;is_bridgingbpjs&quot; , &quot;'&quot; , &quot;);
    var is_bridging_mandiri_inhealth = asuransi_id.find(&quot; , &quot;'&quot; , &quot;:selected&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;is_bridging_mandiri_inhealth&quot; , &quot;'&quot; , &quot;);
    var show_no_asuransi = asuransi_id.find(&quot; , &quot;'&quot; , &quot;:selected&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;show_no_asuransi&quot; , &quot;'&quot; , &quot;);
    var require_no_asuransi = asuransi_id.find(&quot; , &quot;'&quot; , &quot;:selected&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;require_no_asuransi&quot; , &quot;'&quot; , &quot;);

    $(&quot; , &quot;'&quot; , &quot;#nokainhealt&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;);
    $(&quot; , &quot;'&quot; , &quot;#asuransi_bpjs&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;);

    if(show_no_asuransi == &quot;0&quot;){
        $(&quot; , &quot;'&quot; , &quot;.no-asuransi&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;);
        $(&quot; , &quot;'&quot; , &quot;.no-asuransi input&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;);
    }else{
        $(&quot; , &quot;'&quot; , &quot;.no-asuransi&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;);
        if(require_no_asuransi == &quot;1&quot;){
            $(&quot; , &quot;'&quot; , &quot;.icon_asuransi&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;);
            $(&quot; , &quot;'&quot; , &quot;.no-asuransi input&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;, true);
        }else{
            $(&quot; , &quot;'&quot; , &quot;.icon_asuransi&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;);
            $(&quot; , &quot;'&quot; , &quot;.no-asuransi input&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;, false);
        }
        
        $(&quot; , &quot;'&quot; , &quot;#nokainhealt&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;, false);
        if(is_bridgingbpjs == &quot;1&quot;){
            $(&quot; , &quot;'&quot; , &quot;#data_peserta_bpjs&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;);
            $(&quot; , &quot;'&quot; , &quot;#button_bridgingbpjs&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;);
        }else{
            $(&quot; , &quot;'&quot; , &quot;#data_peserta_bpjs&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;);
            $(&quot; , &quot;'&quot; , &quot;#button_bridgingbpjs&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;);
            if (is_bridging_mandiri_inhealth == &quot;1&quot;) {
                if (isPenggunaMandiriInhealth == &quot;&quot; || isPenggunaMandiriInhealth == &quot;0&quot;) {
                    $(&quot; , &quot;'&quot; , &quot;.no-asuransi&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;);
                    $(&quot; , &quot;'&quot; , &quot;.no-asuransi input&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;);
                } else {
                    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[no_asuransi]&quot;]&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
                    $(&quot; , &quot;'&quot; , &quot;.no-asuransi input&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;, false);
                    if(require_no_asuransi == &quot;1&quot;) {
                        $(&quot; , &quot;'&quot; , &quot;#nokainhealt&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;, true);
                    }

                    $(&quot; , &quot;'&quot; , &quot;#asuransi_bpjs&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;);
                    $(&quot; , &quot;'&quot; , &quot;#nokainhealt&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;);
                }
            }
        }
    }
}

function checkAfter(){
    checkNoAsuransi();
    setTimeout(function() {
        setPesertaBpjs();
    }, 1000);
}

function setPesertaBpjs()
{
    var no_asuransi = $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[no_asuransi]&quot;]&quot; , &quot;'&quot; , &quot;);
    if(no_asuransi.val() == &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;){
        return false;
    }
    var nik = $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[nik]&quot;]&quot; , &quot;'&quot; , &quot;);
    if(no_asuransi.val() == &quot;&quot; &amp;&amp; nik.val() == &quot;&quot;){
        alert(&quot; , &quot;'&quot; , &quot;Silahkan isi &lt;b>No Asuransi&lt;/b> atau &lt;b>NIK&lt;/b> untuk mengecek kepesertaan BPJS!&quot; , &quot;'&quot; , &quot;);
        no_asuransi.parent().addClass(&quot; , &quot;'&quot; , &quot;has-error&quot; , &quot;'&quot; , &quot;);
        nik.parent().addClass(&quot; , &quot;'&quot; , &quot;has-error&quot; , &quot;'&quot; , &quot;);
        return false;
    }else{
        no_asuransi.parent().removeClass(&quot; , &quot;'&quot; , &quot;has-error&quot; , &quot;'&quot; , &quot;);
        search = no_asuransi.val();
        if(search == &quot;&quot;){
            search = nik.val();
        }
    }
    no_asuransi.addClass(&quot; , &quot;'&quot; , &quot;ui-autocomplete-loading&quot; , &quot;'&quot; , &quot;);
    var asuransi_id = $(&quot; , &quot;'&quot; , &quot;select[name=&quot;MPasien[asuransi_id]&quot;]&quot; , &quot;'&quot; , &quot;).val();

    $.ajax({
        url: &quot;https://demo.epuskesmas.id/pasien/create&quot;,
        method: &quot;GET&quot;,
        dataType: &quot;json&quot;,
        data: {
            api: &quot; , &quot;'&quot; , &quot;pesertabpjs&quot; , &quot;'&quot; , &quot;,
            search: search,
            asuransi_id: asuransi_id,
        },
        success: function(data) {
            let is_bridgingbpjs_nonactive = data.is_bridgingbpjs_nonactive ?? &quot;false&quot;;
            if (is_bridgingbpjs_nonactive == &quot;active&quot;){

                alert(&quot; , &quot;'&quot; , &quot;Mohon Maaf Settingan Sinkron Asuransi Tersebut sedang Non-aktif Tidak Bisa melakukan Pengecekan No. Asuransi&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;warning&quot; , &quot;'&quot; , &quot;)
                return false;
            }

            no_asuransi.removeClass(&quot; , &quot;'&quot; , &quot;ui-autocomplete-loading&quot; , &quot;'&quot; , &quot;);
            if(data.hasOwnProperty(&quot; , &quot;'&quot; , &quot;metaData&quot; , &quot;'&quot; , &quot;)){
                if(data.metaData.code == 200){
                    peserta = data.response;
                    if(peserta.noKTP == null){
                        alert(&quot;&lt;b>No. KTP (NIK)&lt;\/b> peserta BPJS ini bernilai &lt;b>null&lt;\/b> di response Web Service.&quot; 
                            +&quot;&lt;br> 1. Pastikan pasien BPJS ini tidak memiliki kartu / nomor &lt;b>peserta BPJS ganda&lt;\/b> dan sudah melunasi pembayaran &lt;b>premi&lt;\/b> bulan ini.&quot;
                            +&quot;&lt;br> 2. Infokan kepada pasien BPJS ini agar melakukan cek dan pembaharuan &lt;b>data peserta&lt;\/b> BPJS ke kantor BPJS.&quot;
                            +&quot;&lt;br> 3. Anda tetap bisa mendaftarkan pasien BPJS ini sebagai pasien, namun pastikan untuk mengecek &lt;b>status bpjs&lt;\/b> di aplikasi ePuskesmas dan &lt;b>mengecek data di PCare&lt;\/b>.&quot;
                            +&quot;&lt;br> 4. Info selengkapnya silahkan hubungi &lt;b>petugas BPJS&lt;\/b>.&quot;, &quot;warning&quot;, 0);
                    }
                    alert(&quot;Data peserta BPJS ditemukan!&quot;, &quot;success&quot;);
                    setDataDariApiBpjs(peserta);
                    no_asuransi.parent().removeClass(&quot; , &quot;'&quot; , &quot;has-error&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;has-success&quot; , &quot;'&quot; , &quot;);
                    if($(&quot;#normcheck&quot;).val() == &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;){
                        $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[nik]&quot;]&quot; , &quot;'&quot; , &quot;).trigger(&quot; , &quot;'&quot; , &quot;blur&quot; , &quot;'&quot; , &quot;);
                        if($(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[nik]&quot;]&quot; , &quot;'&quot; , &quot;).val() != &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;){
                            alert(&quot; , &quot;'&quot; , &quot;Pengecekan data pasien di sistem selesai, silahkan cek kembali isian Anda  ..&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;success&quot; , &quot;'&quot; , &quot;);
                        }
                    }
                }else{
                    if(data.metaData.code == 401){
                        alert(&quot;Anda tidak diperkenankan terkoneksi ke Webservice BPJS. Pastikan username dan password di &lt;b>Pengaturan - BPJS&lt;\/b> disesuaikan dengan username dan password PCare terbaru!&lt;br> &lt;span class=&quot; , &quot;'&quot; , &quot;text-danger&quot; , &quot;'&quot; , &quot;> Kode Error: &lt;b>&quot;+data.metaData.code+&quot;&lt;\/b>&lt;\/span>&quot;, &quot;warning&quot;);
                    }else{
                        alert(&quot;Data peserta BPJS tidak ditemukan! Cek kembali &lt;b>No Asuransi&lt;\/b> yang diinput &lt;b>atau&lt;\/b> tunggu beberapa saat hingga Webservice BPJS kembali normal.&lt;br> &lt;span class=&quot; , &quot;'&quot; , &quot;text-danger&quot; , &quot;'&quot; , &quot;> Kode Error: &lt;b>&quot;+data.metaData.code+&quot;&lt;\/b>&lt;\/span>&quot;, &quot;warning&quot;);
                    }
                    no_asuransi.parent().addClass(&quot; , &quot;'&quot; , &quot;has-error&quot; , &quot;'&quot; , &quot;);
                    nik.parent().addClass(&quot; , &quot;'&quot; , &quot;has-error&quot; , &quot;'&quot; , &quot;);
                    $(&quot; , &quot;'&quot; , &quot;#data_peserta_bpjs&quot; , &quot;'&quot; , &quot;).html(&quot;&quot;);
                }
            }else{
                if(data.hasOwnProperty(&quot; , &quot;'&quot; , &quot;o2o&quot; , &quot;'&quot; , &quot;)){
                    alert(data.o2o.message, data.o2o.status);
                }else{
                    alert(&quot;Data peserta BPJS tidak ditemukan! Cek kembali &lt;b>No Asuransi&lt;\/b> yang diinput &lt;b>&quot;, &quot;warning&quot;);
                }
                no_asuransi.parent().addClass(&quot; , &quot;'&quot; , &quot;has-error&quot; , &quot;'&quot; , &quot;);
                nik.parent().addClass(&quot; , &quot;'&quot; , &quot;has-error&quot; , &quot;'&quot; , &quot;);
                $(&quot; , &quot;'&quot; , &quot;#data_peserta_bpjs&quot; , &quot;'&quot; , &quot;).html(&quot;&quot;);
            }
        },
        error: function (xhr, ajaxOptions, thrownError) {
            console.log(xhr.status);
            console.log(thrownError);
            no_asuransi.removeClass(&quot; , &quot;'&quot; , &quot;ui-autocomplete-loading&quot; , &quot;'&quot; , &quot;);
        }
    });
}

function setRequiredForm(reset = false, show = 0)
{
    var requiredInput = [
                        &quot; , &quot;'&quot; , &quot;nik&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;tempat_lahir&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;tanggal_lahir&quot; , &quot;'&quot; , &quot;,
                        &quot; , &quot;'&quot; , &quot;propinsi_id&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;kota_id&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;kecamatan_id&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;kelurahan_id&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;alamat&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;rt&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;rw&quot; , &quot;'&quot; , &quot;,
                        &quot; , &quot;'&quot; , &quot;pekerjaan_id&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;agama&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;pendidikan&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;status_perkawinan&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;status_keluarga&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;warganegara&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;no_hp&quot; , &quot;'&quot; , &quot;
                        ];

    if(global_required!=&quot; , &quot;'&quot; , &quot;undefined&quot; , &quot;'&quot; , &quot;){
        global_required.forEach(function(item,index){
            for (var i=0; i&lt;requiredInput.length; i++)
                if (requiredInput[i] === item){
                    delete requiredInput[i]; 
                }                                    
        });
    }

    if(reset == false){
        requiredInput.forEach(function(item,index){
            var obj = $(&quot; , &quot;'&quot; , &quot;[name=&quot;MPasien[&quot; , &quot;'&quot; , &quot;+item+&quot; , &quot;'&quot; , &quot;]&quot;]&quot; , &quot;'&quot; , &quot;);
            obj.attr(&quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,true);
            obj.parents(&quot; , &quot;'&quot; , &quot;.form-group&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;span[style]&quot; , &quot;'&quot; , &quot;).remove();
            obj.parents(&quot; , &quot;'&quot; , &quot;.form-group&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;label&quot; , &quot;'&quot; , &quot;).append(&quot; , &quot;'&quot; , &quot; &lt;span style=&quot;color:red;&quot;>*&lt;/span>&quot; , &quot;'&quot; , &quot;);
        });    
    }else{
        requiredInput.forEach(function(item,index){
            var obj = $(&quot; , &quot;'&quot; , &quot;[name=&quot;MPasien[&quot; , &quot;'&quot; , &quot;+item+&quot; , &quot;'&quot; , &quot;]&quot;]&quot; , &quot;'&quot; , &quot;);            
            obj.removeAttr(&quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;);
            obj.parents(&quot; , &quot;'&quot; , &quot;.form-group&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;span[style]&quot; , &quot;'&quot; , &quot;).remove();
        });
    }
    
}

function setRequiredFormKtp(reset = false)
{
    var requiredInput = [
                        &quot; , &quot;'&quot; , &quot;ktp_propinsi_id&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;ktp_kota_id&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;ktp_kecamatan_id&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;ktp_kelurahan_id&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;ktp_alamat&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;ktp_rt&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;ktp_rw&quot; , &quot;'&quot; , &quot;
                        ];
    set_up = $(&quot; , &quot;'&quot; , &quot;[name=&quot;MPasien[&quot; , &quot;'&quot; , &quot;+requiredInput[0]+&quot; , &quot;'&quot; , &quot;]&quot;]&quot; , &quot;'&quot; , &quot;).is(&quot; , &quot;'&quot; , &quot;[required]&quot; , &quot;'&quot; , &quot;);
    if(reset==true){
        $(&quot; , &quot;'&quot; , &quot;#collapseAlamatKtp.collapse&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;in&quot; , &quot;'&quot; , &quot;);
    }
    if(set_up == false &amp;&amp; reset == false){
        requiredInput.forEach(function(item,index){
            var obj = $(&quot; , &quot;'&quot; , &quot;[name=&quot;MPasien[&quot; , &quot;'&quot; , &quot;+item+&quot; , &quot;'&quot; , &quot;]&quot;]&quot; , &quot;'&quot; , &quot;);
            obj.attr(&quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,true);
            obj.parents(&quot; , &quot;'&quot; , &quot;.form-group&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;span[style]&quot; , &quot;'&quot; , &quot;).remove();
            obj.parents(&quot; , &quot;'&quot; , &quot;.form-group&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;label&quot; , &quot;'&quot; , &quot;).append(&quot; , &quot;'&quot; , &quot; &lt;span style=&quot;color:red;&quot;>*&lt;/span>&quot; , &quot;'&quot; , &quot;);
        });    
    }else{
        $(&quot; , &quot;'&quot; , &quot;[name=&quot;MPasien[ktp_alamat_berbeda]&quot;]&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;checked&quot; , &quot;'&quot; , &quot;);
        requiredInput.forEach(function(item,index){
            var obj = $(&quot; , &quot;'&quot; , &quot;[name=&quot;MPasien[&quot; , &quot;'&quot; , &quot;+item+&quot; , &quot;'&quot; , &quot;]&quot;]&quot; , &quot;'&quot; , &quot;);
            obj.removeAttr(&quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;);
            obj.parents(&quot; , &quot;'&quot; , &quot;.form-group&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;span[style]&quot; , &quot;'&quot; , &quot;).remove();
        });
    }
}

let isPenggunaMandiriInhealth = &quot;&quot;;

$(&quot;form[form-ajax-print]&quot;).each(function(){
    $(this).on(&quot; , &quot;'&quot; , &quot;submit&quot; , &quot;'&quot; , &quot;,function(event){
        event.preventDefault();
        var button_submit = $(this).find(&quot; , &quot;'&quot; , &quot;button[type=submit]&quot; , &quot;'&quot; , &quot;);
        var button_print = $(this).find(&quot; , &quot;'&quot; , &quot;button[id=button_print]&quot; , &quot;'&quot; , &quot;);
        var button_print_kartu = $(this).find(&quot; , &quot;'&quot; , &quot;button[id=button_print_kartu]&quot; , &quot;'&quot; , &quot;);
        var button_reset = $(this).find(&quot; , &quot;'&quot; , &quot;button[type=reset]&quot; , &quot;'&quot; , &quot;);
        button_submit.addClass(&quot; , &quot;'&quot; , &quot;loading&quot; , &quot;'&quot; , &quot;);

        if(isPenggunaMandiriInhealth == &quot;1&quot;) 
        {
            var is_bridging_mandiri_inhealth = $(&quot; , &quot;'&quot; , &quot;select[name=&quot;MPasien[asuransi_id]&quot;]&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;:selected&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;is_bridging_mandiri_inhealth&quot; , &quot;'&quot; , &quot;);

            if (is_bridging_mandiri_inhealth == &quot;1&quot;) {
                let nokainhealt = $(&quot; , &quot;'&quot; , &quot;input[name=&quot;nokainhealt&quot;]&quot; , &quot;'&quot; , &quot;).val();
                $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[no_asuransi]&quot;]&quot; , &quot;'&quot; , &quot;).val(nokainhealt);                
            }

        }

        $(&quot; , &quot;'&quot; , &quot;#button_saves&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;loading&quot; , &quot;'&quot; , &quot;);
        for (instance in CKEDITOR.instances) {
            CKEDITOR.instances[instance].updateElement();
        }
                
        unsetSeparatorNumber();
        $.ajax({
            url: $(this).attr(&quot; , &quot;'&quot; , &quot;action&quot; , &quot;'&quot; , &quot;),
            method: $(this).attr(&quot; , &quot;'&quot; , &quot;method&quot; , &quot;'&quot; , &quot;),
            dataType: &quot;json&quot;,
            data: $(this).serialize(),
            success: function(data) {
                if(data.activeSwal == &quot;true&quot;){
                    button_submit.removeClass(&quot; , &quot;'&quot; , &quot;loading&quot; , &quot;'&quot; , &quot;);
                    $(&quot;#button_saves&quot;).removeClass(&quot; , &quot;'&quot; , &quot;loading&quot; , &quot;'&quot; , &quot;);
                    Swal.fire({ 
                      title: &quot; , &quot;'&quot; , &quot;Apakah Anda Yakin Merubah Data ?&quot; , &quot;'&quot; , &quot;,
                      type: &quot; , &quot;'&quot; , &quot;info&quot; , &quot;'&quot; , &quot;,
                      customClass: &quot; , &quot;'&quot; , &quot;swal-wide-pasien&quot; , &quot;'&quot; , &quot;,
                      showCancelButton: true,
                      confirmButtonColor: &quot; , &quot;'&quot; , &quot;#3085d6&quot; , &quot;'&quot; , &quot;,
                      cancelButtonColor: &quot; , &quot;'&quot; , &quot;#d33&quot; , &quot;'&quot; , &quot;,
                      confirmButtonText: &quot; , &quot;'&quot; , &quot;Ya&quot; , &quot;'&quot; , &quot;,
                      cancelButtonText: &quot; , &quot;'&quot; , &quot;Tidak&quot; , &quot;'&quot; , &quot;,
                      html: data.message,
                    }).then((result) => {
                      if (result.value) {
                        $(&quot;#cekBpjs&quot;).val(&quot;false&quot;);
                        $(&quot;#button_save&quot;).click();
                      }else if (result.dismiss == &quot;cancel&quot;){
                        $(&quot;#cekBpjs&quot;).val(&quot;default&quot;);
                      }else{
                        $(&quot;#cekBpjs&quot;).val(&quot;default&quot;);
                      }
                      
                    });
                    return false;
                }

                alert(data.message, data.status);
                button_submit.removeClass(&quot; , &quot;'&quot; , &quot;loading&quot; , &quot;'&quot; , &quot;);
                $(&quot; , &quot;'&quot; , &quot;#button_saves&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;loading&quot; , &quot;'&quot; , &quot;);
                if(data.status == &quot; , &quot;'&quot; , &quot;success&quot; , &quot;'&quot; , &quot;){
                    syncPasienKiosk();
                    button_submit.hide();
                    $(&quot; , &quot;'&quot; , &quot;#button_saves&quot; , &quot;'&quot; , &quot;).hide();
                    button_print.show();
                    button_print_kartu.show();
                    button_reset.click(function(){
                        button_submit.show();
                        $(&quot; , &quot;'&quot; , &quot;#button_saves&quot; , &quot;'&quot; , &quot;).show();
                        button_print.hide();
                        button_print.attr(&quot; , &quot;'&quot; , &quot;onclick&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
                        button_print_kartu.hide();
                        button_print_kartu.attr(&quot; , &quot;'&quot; , &quot;onclick&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
                        $( &quot;label[id=created_id]&quot; ).html(&quot; , &quot;'&quot; , &quot;- Otomatis -&quot; , &quot;'&quot; , &quot;);
                    });
                    if(data.id != undefined &amp;&amp; data.id != &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;){
                        $( &quot;label[id=created_id]&quot; ).html(data.id);
                    }
                    if(data.next_url != undefined &amp;&amp; data.next_url != &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;){
                        $(&quot; , &quot;'&quot; , &quot;#button_next&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;href&quot; , &quot;'&quot; , &quot;,data.next_url);
                        $(&quot; , &quot;'&quot; , &quot;#button_next&quot; , &quot;'&quot; , &quot;).show();
                    }
                    if(data.print_url != undefined &amp;&amp; data.print_url != &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;){
                        button_print.attr(&quot; , &quot;'&quot; , &quot;onclick&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;window.open(&quot;&quot; , &quot;'&quot; , &quot;+data.print_url+&quot; , &quot;'&quot; , &quot;&quot;, &quot;&quot;, &quot;left=100,top=100&quot;);&quot; , &quot;'&quot; , &quot;);
                    }

                    if($(&quot; , &quot;'&quot; , &quot;#askep_id&quot; , &quot;'&quot; , &quot;) != null){
                        $(&quot; , &quot;'&quot; , &quot;#button_askep&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;href&quot; , &quot;'&quot; , &quot;, data.next_url+&quot; , &quot;'&quot; , &quot;?pengkajian_keperawatan_id=&quot; , &quot;'&quot; , &quot;+$(&quot; , &quot;'&quot; , &quot;#askep_id&quot; , &quot;'&quot; , &quot;).val());
                    }

                    if(data.print_url_kartu != undefined &amp;&amp; data.print_url_kartu != &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;){
                        button_print_kartu.attr(&quot; , &quot;'&quot; , &quot;onclick&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;window.open(&quot;&quot; , &quot;'&quot; , &quot;+data.print_url_kartu+&quot; , &quot;'&quot; , &quot;&quot;, &quot;&quot;, &quot;left=100,top=100,width=420,height=260&quot;);&quot; , &quot;'&quot; , &quot;);
                    }

                    if(data.show_url != undefined &amp;&amp; data.show_url != &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;){
                        var url = data.show_url;
                        $.pjax({url: url, container: &quot; , &quot;'&quot; , &quot;#content&quot; , &quot;'&quot; , &quot;});
                    }
                }
                setSeparatorNumber();
            },
            error: function (xhr, ajaxOptions, thrownError) {
                alert(&quot;Terjadi kesalahan sistem, silahkan hubungi team support kami!&quot;, &quot;danger&quot;);
                button_submit.removeClass(&quot; , &quot;'&quot; , &quot;loading&quot; , &quot;'&quot; , &quot;);
                $(&quot; , &quot;'&quot; , &quot;#button_saves&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;loading&quot; , &quot;'&quot; , &quot;);
                console.log(xhr.status);
                console.log(thrownError);
                setSeparatorNumber();
            }
        });
    });
});

function checkNoAsuransi(){
    checkAsuransiVal();
    var no_asuransi = $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[no_asuransi]&quot;]&quot; , &quot;'&quot; , &quot;).val();
    var asuransi_id = $(&quot; , &quot;'&quot; , &quot;select[name=&quot;MPasien[asuransi_id]&quot;]&quot; , &quot;'&quot; , &quot;).val();
    $.ajax({
        url: &quot;https://demo.epuskesmas.id/pasien/create&quot;,
        method: &quot;GET&quot;,
        dataType: &quot;json&quot;,
        data: {
            check: &quot; , &quot;'&quot; , &quot;no_asuransi&quot; , &quot;'&quot; , &quot;,
            search: {
                &quot; , &quot;'&quot; , &quot;no_asuransi&quot; , &quot;'&quot; , &quot;:no_asuransi,
                &quot; , &quot;'&quot; , &quot;asuransi_id&quot; , &quot;'&quot; , &quot;:asuransi_id,
                &quot; , &quot;'&quot; , &quot;id&quot; , &quot;'&quot; , &quot;:&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;
            }
        },
        success: function(data) {
            if(data.status == &quot; , &quot;'&quot; , &quot;success&quot; , &quot;'&quot; , &quot;){
                $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[no_asuransi]&quot;]&quot; , &quot;'&quot; , &quot;).parent().addClass(&quot; , &quot;'&quot; , &quot;has-success&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;has-error&quot; , &quot;'&quot; , &quot;);
            }else if(data.length &lt;= 0){
                $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[no_asuransi]&quot;]&quot; , &quot;'&quot; , &quot;).parent().addClass(&quot; , &quot;'&quot; , &quot;has-error&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;has-success&quot; , &quot;'&quot; , &quot;);
            }else{
                alert(data.message, data.status);
                $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[no_asuransi]&quot;]&quot; , &quot;'&quot; , &quot;).parent().addClass(&quot; , &quot;'&quot; , &quot;has-error&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;has-success&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;);
                $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[no_asuransi]&quot;]&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
                $(&quot; , &quot;'&quot; , &quot;#data_peserta_bpjs&quot; , &quot;'&quot; , &quot;).html(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
            }
            
        },
        error: function (xhr, ajaxOptions, thrownError) {
            console.log(xhr.status);
            console.log(thrownError);
            $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[no_asuransi]&quot;]&quot; , &quot;'&quot; , &quot;).parent().removeClass(&quot; , &quot;'&quot; , &quot;has-success&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;has-error&quot; , &quot;'&quot; , &quot;);
        }
    });

}

function checkNik(nik){
    if(&quot;e-Puskesmas&quot; == &quot; , &quot;'&quot; , &quot;e-Clinic&quot; , &quot;'&quot; , &quot;) {
      var Nik = $(&quot;input[name=&quot; , &quot;'&quot; , &quot;MPasien[nik]&quot; , &quot;'&quot; , &quot;]&quot;);
      Nik.attr(&quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;, true); 
      Nik.parents(&quot; , &quot;'&quot; , &quot;.form-group&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;span[style]&quot; , &quot;'&quot; , &quot;).remove();
      Nik.parents(&quot; , &quot;'&quot; , &quot;.form-group&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;label&quot; , &quot;'&quot; , &quot;).append(&quot; , &quot;'&quot; , &quot; &lt;span style=&quot;color:red;&quot;>*&lt;/span>&quot; , &quot;'&quot; , &quot;);
      var radio = $(&quot; , &quot;'&quot; , &quot;#radioNik&quot; , &quot;'&quot; , &quot;);
      radio.attr(&quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;, false);
      radio.prop(&quot; , &quot;'&quot; , &quot;checked&quot; , &quot;'&quot; , &quot;, false);
      radio.attr(&quot; , &quot;'&quot; , &quot;checked&quot; , &quot;'&quot; , &quot;, false);
      radio.parent(&quot; , &quot;'&quot; , &quot;label&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;span[style]&quot; , &quot;'&quot; , &quot;).remove();
    }
    if(nik.length != 16){
        var message = &quot;&quot;;
        if(nik.length &lt; 16){
            message += &quot;NIK kurang dari 16 digit!&quot;;
        }else{
            message += &quot;NIK lebih dari 16 digit!&quot;;
        }
        message +=&quot;&lt;br>Silahkan periksa kembali dan pastikan NIK yang diinput sesuai dengan yang tercantum di KTP atau KK!&quot;;
        alert(message);
        $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[nik]&quot;]&quot; , &quot;'&quot; , &quot;).parent().addClass(&quot; , &quot;'&quot; , &quot;has-error&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;has-success&quot; , &quot;'&quot; , &quot;);
        return false;
    }
        $.ajax({
        url: &quot;https://demo.epuskesmas.id/pasien/create&quot;,
        method: &quot;GET&quot;,
        dataType: &quot;json&quot;,
        data: {
            check: &quot; , &quot;'&quot; , &quot;nik&quot; , &quot;'&quot; , &quot;,
            search: {
                &quot; , &quot;'&quot; , &quot;nik&quot; , &quot;'&quot; , &quot;:nik,
                &quot; , &quot;'&quot; , &quot;id&quot; , &quot;'&quot; , &quot;:&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;
            }
        },
        success: function(data) {
                            if(data.puskesmas == true){
                    window.location = &quot;https://demo.epuskesmas.id/pasien/edit/&quot;+data.result.id;
                }else{
                    if(data.result){
                        setDataDariKota(data.result);
                    }
                }
                return false;
                        if(data.status == &quot; , &quot;'&quot; , &quot;success&quot; , &quot;'&quot; , &quot;){
                $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[nik]&quot;]&quot; , &quot;'&quot; , &quot;).parent().addClass(&quot; , &quot;'&quot; , &quot;has-success&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;has-error&quot; , &quot;'&quot; , &quot;);
            }else{
                alert(data.message, data.status);
                $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[nik]&quot;]&quot; , &quot;'&quot; , &quot;).parent().addClass(&quot; , &quot;'&quot; , &quot;has-error&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;has-success&quot; , &quot;'&quot; , &quot;);
            }
        },
        error: function (xhr, ajaxOptions, thrownError) {
            console.log(xhr.status);
            console.log(thrownError);
            $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[nik]&quot;]&quot; , &quot;'&quot; , &quot;).parent().removeClass(&quot; , &quot;'&quot; , &quot;has-success&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;has-error&quot; , &quot;'&quot; , &quot;);
        }
    });
}

function getDisdukcapil(){
    var nik = $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[nik]&quot;]&quot; , &quot;'&quot; , &quot;);
    $.ajax({
        url: &quot;https://demo.epuskesmas.id/pasien/create&quot;,
        method: &quot;GET&quot;,
        dataType: &quot;json&quot;,
        data: {
            getData: &quot; , &quot;'&quot; , &quot;disdukcapil&quot; , &quot;'&quot; , &quot;,
            search: {
                &quot; , &quot;'&quot; , &quot;nik&quot; , &quot;'&quot; , &quot;:nik.val(),
            }
        },
        success: function(data) {
            setRequiredForm(true);
            $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[verified_by]&quot;]&quot; , &quot;'&quot; , &quot;).prop(&quot;checked&quot;, false);
            if(data.kesalahan){
                alert(data.kesalahan);
                $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[nik]&quot;]&quot; , &quot;'&quot; , &quot;).parent().addClass(&quot; , &quot;'&quot; , &quot;has-error&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;has-success&quot; , &quot;'&quot; , &quot;);
            }else{
                alert(&quot;Biodata Disdukcapil ditemukan!&quot;, &quot;success&quot;);
                setRequiredForm(false);
                $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[verified_by]&quot;]&quot; , &quot;'&quot; , &quot;).prop(&quot;checked&quot;, true);
                setDataDisdukcapil(data);
                $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[nik]&quot;]&quot; , &quot;'&quot; , &quot;).parent().addClass(&quot; , &quot;'&quot; , &quot;has-success&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;has-error&quot; , &quot;'&quot; , &quot;);
            }
        },
        error: function (xhr, ajaxOptions, thrownError) {
            console.log(xhr.status);
            console.log(thrownError);
            $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[nik]&quot;]&quot; , &quot;'&quot; , &quot;).parent().removeClass(&quot; , &quot;'&quot; , &quot;has-success&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;has-error&quot; , &quot;'&quot; , &quot;);
        }
    });
}


function saveAtas(){
    $(&quot; , &quot;'&quot; , &quot;#button_saves&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;loading&quot; , &quot;'&quot; , &quot;);
    $(&quot; , &quot;'&quot; , &quot;#button_save&quot; , &quot;'&quot; , &quot;).click();
}

function setDataDisdukcapil(data)
{
    setWilayah(data);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[nama]&quot;]&quot; , &quot;'&quot; , &quot;).val(data.nama);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[jenis_kelamin]&quot;][value=&quot;&quot; , &quot;'&quot; , &quot;+data.jenis_kelamin+&quot; , &quot;'&quot; , &quot;&quot;]&quot; , &quot;'&quot; , &quot;).prop(&quot;checked&quot;,true);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[tanggal_lahir]&quot;]&quot; , &quot;'&quot; , &quot;).val(data.tanggal_lahir);
    $(&quot; , &quot;'&quot; , &quot;select[name=&quot;MPasien[gol_darah]&quot;]&quot; , &quot;'&quot; , &quot;).val(data.gol_darah);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[no_hp]&quot;]&quot; , &quot;'&quot; , &quot;).val(data.no_hp);
    $(&quot; , &quot;'&quot; , &quot;select[name=&quot;MPasien[status_keluarga]&quot;]&quot; , &quot;'&quot; , &quot;).val(data.status_keluarga);
    $(&quot; , &quot;'&quot; , &quot;select[name=&quot;MPasien[status_perkawinan]&quot;]&quot; , &quot;'&quot; , &quot;).val(data.status_perkawinan);
    $(&quot; , &quot;'&quot; , &quot;select[name=&quot;MPasien[pendidikan]&quot;]&quot; , &quot;'&quot; , &quot;).val(data.pendidikan);
    $(&quot; , &quot;'&quot; , &quot;select[name=&quot;MPasien[agama]&quot;]&quot; , &quot;'&quot; , &quot;).val(data.agama);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;pekerjaan_nama&quot;]&quot; , &quot;'&quot; , &quot;).val(data.pekerjaan_nama);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[pekerjaan_id]&quot;]&quot; , &quot;'&quot; , &quot;).val(data.pekerjaan_id);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[instansi]&quot;]&quot; , &quot;'&quot; , &quot;).val(data.instansi);
    $(&quot; , &quot;'&quot; , &quot;textarea[name=&quot;MPasien[alamat]&quot;]&quot; , &quot;'&quot; , &quot;).val(data.alamat);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[rt]&quot;]&quot; , &quot;'&quot; , &quot;).val(data.rt);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[rw]&quot;]&quot; , &quot;'&quot; , &quot;).val(data.rw);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[nama_ayah]&quot;]&quot; , &quot;'&quot; , &quot;).val(data.nama_ayah);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[nama_ibu]&quot;]&quot; , &quot;'&quot; , &quot;).val(data.nama_ibu);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[tempat_lahir]&quot;]&quot; , &quot;'&quot; , &quot;).val(data.tempat_lahir);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[no_kk]&quot;]&quot; , &quot;'&quot; , &quot;).val(data.no_kk);
    $(&quot; , &quot;'&quot; , &quot;select[name=&quot;MPasien[warganegara]&quot;]&quot; , &quot;'&quot; , &quot;).val(data.warganegara);
}

    var global_required = [&quot;&quot;] ;


$(document).ready(function(){

        setAsuransi();


    if($(&quot; , &quot;'&quot; , &quot;#checked_verifikasi&quot; , &quot;'&quot; , &quot;).is(&quot; , &quot;'&quot; , &quot;:checked&quot; , &quot;'&quot; , &quot;)) {
        setRequiredForm(false);
    }else{
        setRequiredForm(true);
    }

    $(&quot;input[name=&quot; , &quot;'&quot; , &quot;MPasien[no_dok_rm]&quot; , &quot;'&quot; , &quot;]&quot;).on(&quot; , &quot;'&quot; , &quot;blur&quot; , &quot;'&quot; , &quot;, function () {
        $.ajax({
            url: &quot;https://demo.epuskesmas.id/pasien/create&quot;,
            method: &quot;GET&quot;,
            dataType: &quot;json&quot;,
            data: {
                no_dok_rm: this.value
            },
            success: function(data) {
                if (data.message != &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;) {
                    Swal.fire({ 
                        title: &quot; , &quot;'&quot; , &quot;Perhatian!&quot; , &quot;'&quot; , &quot;,
                        type: &quot; , &quot;'&quot; , &quot;info&quot; , &quot;'&quot; , &quot;,
                        customClass: &quot; , &quot;'&quot; , &quot;swal-wide-pasien&quot; , &quot;'&quot; , &quot;,
                        showCancelButton: true,
                        confirmButtonColor: &quot; , &quot;'&quot; , &quot;#3085d6&quot; , &quot;'&quot; , &quot;,
                        cancelButtonColor: &quot; , &quot;'&quot; , &quot;#d33&quot; , &quot;'&quot; , &quot;,
                        confirmButtonText: &quot; , &quot;'&quot; , &quot;Ya&quot; , &quot;'&quot; , &quot;,
                        cancelButtonText: &quot; , &quot;'&quot; , &quot;Tidak&quot; , &quot;'&quot; , &quot;,
                        html: data.message,
                    }).then((result) => {
                        if (result.value) {
                            $(&quot;input[name=&quot; , &quot;'&quot; , &quot;MPasien[no_dok_rm]&quot; , &quot;'&quot; , &quot;]&quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;).focus();
                        }
                    });
                }
            },
            error: function (xhr, ajaxOptions, thrownError) {
                console.log(xhr.status);
                console.log(thrownError);
            }
        });
    });
});

$(&quot; , &quot;'&quot; , &quot;#checked_verifikasi&quot; , &quot;'&quot; , &quot;).click( function(){
    if($(this).is(&quot; , &quot;'&quot; , &quot;:checked&quot; , &quot;'&quot; , &quot;)) {
        setRequiredForm(false);
    }else{
        setRequiredForm(true);
    }
});

function checkAsuransiVal(){
    var asuransi_id = $(&quot; , &quot;'&quot; , &quot;select[name=&quot;MPasien[asuransi_id]&quot;]&quot; , &quot;'&quot; , &quot;);
    var is_bridgingbpjs = asuransi_id.find(&quot; , &quot;'&quot; , &quot;:selected&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;is_bridgingbpjs&quot; , &quot;'&quot; , &quot;);
    var no_asuransi = $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[no_asuransi]&quot;]&quot; , &quot;'&quot; , &quot;).val();
    var pad = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
    if (is_bridgingbpjs == &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot; &amp;&amp; no_asuransi.length &lt; 13 &amp;&amp; no_asuransi != &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;) {
        for (i=1; i &lt;= (13 - no_asuransi.length); i++) {
            pad += &quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;;
        }
    }
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[no_asuransi]&quot;]&quot; , &quot;'&quot; , &quot;).val(`${pad}${no_asuransi}`);
}

function printLabel(id, tipe) {
    window.open(`https://demo.epuskesmas.id/pasien/printlabel/${id}/${tipe}`, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;left=0,top=0&quot; , &quot;'&quot; , &quot;);
}

$(&quot;select.warganegara&quot;).change(function(){
        var selected = $(this).children(&quot;option:selected&quot;).val();
        if (selected == &quot;INDONESIA&quot;) {
            $(&quot;input[name=&quot; , &quot;'&quot; , &quot;MPasien[negara_asal]&quot; , &quot;'&quot; , &quot;]&quot;).val(&quot; , &quot;'&quot; , &quot;INDONESIA&quot; , &quot;'&quot; , &quot;);
            if (&quot;e-Puskesmas&quot; == &quot; , &quot;'&quot; , &quot;e-Clinic&quot; , &quot;'&quot; , &quot;) {
              var paspor = $(&quot;input[name=&quot; , &quot;'&quot; , &quot;MPasien[no_paspor]&quot; , &quot;'&quot; , &quot;]&quot;);
              paspor.attr(&quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;, false);
              paspor.parents(&quot; , &quot;'&quot; , &quot;.form-group&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;span[style]&quot; , &quot;'&quot; , &quot;).remove();
              paspor.parents(&quot; , &quot;'&quot; , &quot;.form-group&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;);
              var nik = $(&quot;input[name=&quot; , &quot;'&quot; , &quot;MPasien[nik]&quot; , &quot;'&quot; , &quot;]&quot;);
              nik.parents(&quot; , &quot;'&quot; , &quot;.form-group&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;);
              nik.attr(&quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;, true);
              nik.parents(&quot; , &quot;'&quot; , &quot;.form-group&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;span[style]&quot; , &quot;'&quot; , &quot;).remove();
              nik.parents(&quot; , &quot;'&quot; , &quot;.form-group&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;label&quot; , &quot;'&quot; , &quot;).append(&quot; , &quot;'&quot; , &quot; &lt;span style=&quot;color:red;&quot;>*&lt;/span>&quot; , &quot;'&quot; , &quot;);
              var radnik = $(&quot; , &quot;'&quot; , &quot;#radioNik&quot; , &quot;'&quot; , &quot;);
              radnik.parents(&quot; , &quot;'&quot; , &quot;.form-group&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;);
              setRequiredFormAlamat(true);
            }
        } else if (selected == &quot;ASING&quot;){
            $(&quot;input[name=&quot; , &quot;'&quot; , &quot;MPasien[negara_asal]&quot; , &quot;'&quot; , &quot;]&quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
            if (&quot;e-Puskesmas&quot; == &quot; , &quot;'&quot; , &quot;e-Clinic&quot; , &quot;'&quot; , &quot;) {
              var paspor = $(&quot;input[name=&quot; , &quot;'&quot; , &quot;MPasien[no_paspor]&quot; , &quot;'&quot; , &quot;]&quot;);
              paspor.parents(&quot; , &quot;'&quot; , &quot;.form-group&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;);
              paspor.parents(&quot; , &quot;'&quot; , &quot;.form-group&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;label&quot; , &quot;'&quot; , &quot;).append(&quot; , &quot;'&quot; , &quot; &lt;span style=&quot;color:red;&quot;>*&lt;/span>&quot; , &quot;'&quot; , &quot;);
              paspor.attr(&quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;, true);
              var nik = $(&quot;input[name=&quot; , &quot;'&quot; , &quot;MPasien[nik]&quot; , &quot;'&quot; , &quot;]&quot;);
              nik.val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
              nik.attr(&quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;, false);
              nik.parents(&quot; , &quot;'&quot; , &quot;.form-group&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;span[style]&quot; , &quot;'&quot; , &quot;).remove();
              nik.parents(&quot; , &quot;'&quot; , &quot;.form-group&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;);
              var radnik = $(&quot; , &quot;'&quot; , &quot;#radioNik&quot; , &quot;'&quot; , &quot;);
              radnik.attr(&quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;, false);
              radnik.attr(&quot; , &quot;'&quot; , &quot;checked&quot; , &quot;'&quot; , &quot;, false);
              radnik.prop(&quot; , &quot;'&quot; , &quot;checked&quot; , &quot;'&quot; , &quot;, false);
              radnik.parents(&quot; , &quot;'&quot; , &quot;.radio-inline&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;span[style]&quot; , &quot;'&quot; , &quot;).remove();
              radnik.parents(&quot; , &quot;'&quot; , &quot;.form-group&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;);
              setRequiredFormAlamat(false);
            }
        } else {
            $(&quot;input[name=&quot; , &quot;'&quot; , &quot;MPasien[negara_asal]&quot; , &quot;'&quot; , &quot;]&quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
            if (&quot;e-Puskesmas&quot; == &quot; , &quot;'&quot; , &quot;e-Clinic&quot; , &quot;'&quot; , &quot;) {
              var paspor = $(&quot;input[name=&quot; , &quot;'&quot; , &quot;MPasien[no_paspor]&quot; , &quot;'&quot; , &quot;]&quot;);
              paspor.attr(&quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;, false);
              paspor.parents(&quot; , &quot;'&quot; , &quot;.form-group&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;span[style]&quot; , &quot;'&quot; , &quot;).remove();
              paspor.parents(&quot; , &quot;'&quot; , &quot;.form-group&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;);
              var nik = $(&quot;input[name=&quot; , &quot;'&quot; , &quot;MPasien[nik]&quot; , &quot;'&quot; , &quot;]&quot;);
              nik.val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
              nik.attr(&quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;, false);
              nik.parents(&quot; , &quot;'&quot; , &quot;.form-group&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;span[style]&quot; , &quot;'&quot; , &quot;).remove();
              nik.parents(&quot; , &quot;'&quot; , &quot;.form-group&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;);
              var radnik = $(&quot; , &quot;'&quot; , &quot;#radioNik&quot; , &quot;'&quot; , &quot;);
              radnik.attr(&quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;, false);
              radnik.attr(&quot; , &quot;'&quot; , &quot;checked&quot; , &quot;'&quot; , &quot;, false);
              radnik.prop(&quot; , &quot;'&quot; , &quot;checked&quot; , &quot;'&quot; , &quot;, false);
              radnik.parents(&quot; , &quot;'&quot; , &quot;.radio-inline&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;span[style]&quot; , &quot;'&quot; , &quot;).remove();
              radnik.parents(&quot; , &quot;'&quot; , &quot;.form-group&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;);
              setRequiredFormAlamat(false);
            }
        }
    });

function checkradionik(obj)
{
  var radio = $(&quot; , &quot;'&quot; , &quot;#radioNik&quot; , &quot;'&quot; , &quot;);
  var nik = $(&quot;input[name=&quot; , &quot;'&quot; , &quot;MPasien[nik]&quot; , &quot;'&quot; , &quot;]&quot;);
  if ($(obj).is(&quot; , &quot;'&quot; , &quot;:checked&quot; , &quot;'&quot; , &quot;)) {
    nik.val(&quot;&quot;);
    nik.attr(&quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;, false);
    nik.parents(&quot; , &quot;'&quot; , &quot;.form-group&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;span[style]&quot; , &quot;'&quot; , &quot;).remove();
    radio.attr(&quot; , &quot;'&quot; , &quot;checked&quot; , &quot;'&quot; , &quot;, true);
  } else {
    radio.attr(&quot; , &quot;'&quot; , &quot;checked&quot; , &quot;'&quot; , &quot;, false);
    nik.val(&quot;&quot;);
    nik.attr(&quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;, true);
    nik.parents(&quot; , &quot;'&quot; , &quot;.form-group&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;span[style]&quot; , &quot;'&quot; , &quot;).remove();
    nik.parents(&quot; , &quot;'&quot; , &quot;.form-group&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;label&quot; , &quot;'&quot; , &quot;).append(&quot; , &quot;'&quot; , &quot; &lt;span style=&quot;color:red;&quot;>*&lt;/span>&quot; , &quot;'&quot; , &quot;);
  }
}

function setRequiredFormAlamat(reset = false, show = 0)
{
  var requiredInput = [&quot; , &quot;'&quot; , &quot;propinsi_id&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;kota_id&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;kecamatan_id&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;kelurahan_id&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;alamat&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;rt&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;rw&quot; , &quot;'&quot; , &quot;];
  if(reset == true){
  requiredInput.forEach(function(item,index){
    var obj = $(&quot; , &quot;'&quot; , &quot;[name=&quot;MPasien[&quot; , &quot;'&quot; , &quot;+item+&quot; , &quot;'&quot; , &quot;]&quot;]&quot; , &quot;'&quot; , &quot;);
    obj.attr(&quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,true);
    obj.parents(&quot; , &quot;'&quot; , &quot;.form-group&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;span[style]&quot; , &quot;'&quot; , &quot;).remove();
    obj.parents(&quot; , &quot;'&quot; , &quot;.form-group&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;label&quot; , &quot;'&quot; , &quot;).append(&quot; , &quot;'&quot; , &quot; &lt;span style=&quot;color:red;&quot;>*&lt;/span>&quot; , &quot;'&quot; , &quot;);
  });    
  }else{
    requiredInput.forEach(function(item,index){
      var obj = $(&quot; , &quot;'&quot; , &quot;[name=&quot;MPasien[&quot; , &quot;'&quot; , &quot;+item+&quot; , &quot;'&quot; , &quot;]&quot;]&quot; , &quot;'&quot; , &quot;);            
      obj.removeAttr(&quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;);
      obj.parents(&quot; , &quot;'&quot; , &quot;.form-group&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;span[style]&quot; , &quot;'&quot; , &quot;).remove();
    });
  }
}


    
        SUMATERA BARAT SUMATERA SELATAN SUMATERA UTARA 3 results are available, use up and down arrow keys to navigate.SUMATERA BARATSUMATERA SELATANSUMATERA UTARASUMATERA UTARAKABUPATEN KARO SUMATERA UTARA1 result is available, use up and down arrow keys to navigate.KABUPATEN KARO
        
            
                 
            
        
        
        
        
            
                
                    
                        ×
                        Memuat... 
                    
                    
                        
                        
                        
                    
                
            
        

        
            
                
                    
                        ×
                        Memuat... 
                    
                    
                        
                        
                        
                        
                    
                
            
        

        
            
                
                    
                        ×
                        Memuat... 
                    
                    
                        
                        
                        
                    
                
            
        
        
            
                
                    
                        ×
                          Notifikasi
                    
                    
                        
                        
                    
                
            
        
                                
                 Webservice BPJS sedang Gangguan    
                X
            
                                        
	#modalBantuan {
        position: fixed;
        z-index: 99998; 
        height: 52px; 
        width: 52px;
        background-color: #25d366;
        text-align: center;
        bottom: 5%;
        left: 2%;
        border-radius: 50%;
        font-size: 40px;
    }

    #modalBantuan > i {
        color: white;
        padding-top: 5px;
    }






    function openBantuan(){
        Swal.fire({
            title: &quot; , &quot;'&quot; , &quot;Apakah anda membutuhkan bantuan dari Team Customer Support?&quot; , &quot;'&quot; , &quot;,
            type: &quot; , &quot;'&quot; , &quot;info&quot; , &quot;'&quot; , &quot;,
            showCancelButton: true,
            confirmButtonColor: &quot; , &quot;'&quot; , &quot;#3085d6&quot; , &quot;'&quot; , &quot;,
            cancelButtonColor: &quot; , &quot;'&quot; , &quot;#d33&quot; , &quot;'&quot; , &quot;,
            confirmButtonText: &quot; , &quot;'&quot; , &quot;Ya&quot; , &quot;'&quot; , &quot;,
            cancelButtonText: &quot; , &quot;'&quot; , &quot;Tidak&quot; , &quot;'&quot; , &quot;
        }).then((result) => {
            if (result.value) {
                let notif_wa = &quot;https://api.whatsapp.com/send/?phone=6281277073300&amp;text=Hi,%20Saya%20pengguna%20ePuskesmas:%20,%20%20(darel)%20-%20P0000000001%20PUSKESMAS1%20-%20KABUPATEN%20BANDUNG%20memerlukan%20bantuan%20anda!&amp;app_absent=0&quot;
                window.open(notif_wa);
            }
        });
    }

    $(&quot;#modalBantuan&quot;).mousedown(function() {
        isDragging = false;
    }).mousemove(function() {
        isDragging = true;
    }).mouseup(function() {
        var wasDragging = isDragging;
        isDragging = false;
        if (!wasDragging) {
            openBantuan();
        }
    });

    dragElement(document.getElementById(&quot;modalBantuan&quot;));

    function dragElement(elmnt) {
      var pos1 = 0, pos2 = 0, pos3 = 0, pos4 = 0;
      if (document.getElementById(elmnt.id + &quot;header&quot;)) {
        document.getElementById(elmnt.id + &quot;header&quot;).onmousedown = dragMouseDown;
      } else {
        elmnt.onmousedown = dragMouseDown;
      }

      function dragMouseDown(e) {
        e = e || window.event;
        e.preventDefault();
        pos3 = e.clientX;
        pos4 = e.clientY;
        document.onmouseup = closeDragElement;
        document.onmousemove = elementDrag;
      }

      function elementDrag(e) {
        e = e || window.event;
        e.preventDefault();
        pos1 = pos3 - e.clientX;
        pos2 = pos4 - e.clientY;
        pos3 = e.clientX;
        pos4 = e.clientY;
        elmnt.style.top = (elmnt.offsetTop - pos2) + &quot;px&quot;;
        elmnt.style.left = (elmnt.offsetLeft - pos1) + &quot;px&quot;;
      }

      function closeDragElement() {
        document.onmouseup = null;
        document.onmousemove = null;
      }
    }

    var modalBantuanDrag = document.getElementById(&quot; , &quot;'&quot; , &quot;modalBantuan&quot; , &quot;'&quot; , &quot;);
    modalBantuanDrag.addEventListener(&quot; , &quot;'&quot; , &quot;touchmove&quot; , &quot;'&quot; , &quot;, function(event) {
      event.preventDefault();
      if (event.targetTouches.length == 1) {
        var touch = event.targetTouches[0];
        modalBantuanDrag.style.left = touch.pageX + &quot; , &quot;'&quot; , &quot;px&quot; , &quot;'&quot; , &quot;;
        modalBantuanDrag.style.top = touch.pageY + &quot; , &quot;'&quot; , &quot;px&quot; , &quot;'&quot; , &quot;;
      }
    }, {passive: true});
                
            $(document).ready(function(){
                $(window).scroll(function () {
                    $(&quot; , &quot;'&quot; , &quot;#icon_back&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;style&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;margin-top:&quot; , &quot;'&quot; , &quot;+(parseInt(innerHeight)-150)+&quot; , &quot;'&quot; , &quot;px;margin-left:&quot; , &quot;'&quot; , &quot;+(parseInt(innerWidth)-70)+&quot; , &quot;'&quot; , &quot;px;z-index:1000;&quot; , &quot;'&quot; , &quot;);
                    if ($(this).scrollTop() > 125) {
                        $(&quot; , &quot;'&quot; , &quot;#top-link-block&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;);
                        $(&quot; , &quot;'&quot; , &quot;#top-link-block&quot; , &quot;'&quot; , &quot;).fadeIn();
                    }else{
                        $(&quot; , &quot;'&quot; , &quot;#top-link-block&quot; , &quot;'&quot; , &quot;).fadeOut();
                    }                    
                });

                let message_skrining= &quot; , &quot;'&quot; , &quot;null&quot; , &quot;'&quot; , &quot;;
                if(message_skrining == &quot;error_message&quot;){

                    let text_message = &quot; , &quot;'&quot; , &quot;null&quot; , &quot;'&quot; , &quot;;
                    let text_status = &quot; , &quot;'&quot; , &quot;null&quot; , &quot;'&quot; , &quot;;
                    if(text_status == &quot; , &quot;'&quot; , &quot;null&quot; , &quot;'&quot; , &quot;){
                        text_status = &quot; , &quot;'&quot; , &quot;error&quot; , &quot;'&quot; , &quot;;
                    }
                    if(text_message != &quot; , &quot;'&quot; , &quot;null&quot; , &quot;'&quot; , &quot;){
                        Swal.fire(
                          text_message,
                          &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;,
                          text_status
                        )
                    }
                }

                $(&quot; , &quot;'&quot; , &quot;[data-toggle=&quot;tooltip&quot;]&quot; , &quot;'&quot; , &quot;).tooltip();


                                                                                        let stt= localStorage.getItem(&quot;count_notif&quot;);
                            if(stt !== &quot;undefined&quot; &amp;&amp; stt !== &quot;0&quot; &amp;&amp; stt !== &quot;null&quot; ){
                                $(&quot;#kolom_gagal_bridging&quot;).html(stt);   
                            }
                                                                            
            });
            
            function aksipesan(){
                Swal.fire({
                    text: &quot;Apakah Anda ingin tutup pesan Webservice bpjs dan tidak di tampilkan lagi ?&quot;,
                    type: &quot; , &quot;'&quot; , &quot;warning&quot; , &quot;'&quot; , &quot;,
                    showCancelButton: true,
                    confirmButtonColor: &quot; , &quot;'&quot; , &quot;#3085d6&quot; , &quot;'&quot; , &quot;,
                    cancelButtonColor: &quot; , &quot;'&quot; , &quot;#d33&quot; , &quot;'&quot; , &quot;,
                    confirmButtonText: &quot; , &quot;'&quot; , &quot;Ya&quot; , &quot;'&quot; , &quot;,
                    cancelButtonText: &quot; , &quot;'&quot; , &quot;Tutup Sekarang Saja&quot; , &quot;'&quot; , &quot;
                }).then((result) => {
                    if(result.value){
                        setSessionWebSocket(1, &quot;indikatorbpjs&quot;);
                        $(&quot; , &quot;'&quot; , &quot;#notif-connection-bpjs&quot; , &quot;'&quot; , &quot;).hide();
                    }

                    if (result.dismiss == &quot;cancel&quot;) {
                        $(&quot; , &quot;'&quot; , &quot;#notif-connection-bpjs&quot; , &quot;'&quot; , &quot;).hide();
                    }
                });
            }

                        function callAntreanKiosk(nomorAntrean, ruangan_code){
                                return false;
            }
            
            function deletePasienKiosk(pasien_id){
                                return false;
            }

            function deletePendaftaranKiosk(pendaftaran_id){
                                return false;
            }

            function syncPasienKiosk(){
                                return false;
            }

            function syncPendaftaranKiosk(){
                                return false;
            }
        
        
            var date = new Date();
            var tanggal = new Date(date.getFullYear(), date.getMonth(), date.getDate(), 23, 59, 59, 59);
            var bulan = new Date(date.getFullYear(), date.getMonth(), 1, 0, 0, 0);
            var tahun = new Date(date.getFullYear(), 1, 0, 0, 0);
                            var tanggal_min = new Date(date.getFullYear(), date.getMonth(), date.getDate(), 00, 00, 00, 00);
                        var sistole_diastole_anamnesa = &quot;&quot;;
            var default_sistole = &quot;&quot;;
            var default_diastole = &quot;&quot;;
        
        
                    
            
                
            function showUpdateLog(from_menu = false)
            {
                $.ajax({
                    url: &quot;https://demo.epuskesmas.id/updatelog/showupdatelog&quot;,
                    method: &quot; , &quot;'&quot; , &quot;GET&quot; , &quot;'&quot; , &quot;,
                    dataType: &quot;json&quot;,
                    data: {
                        _token: &quot;1Qzy4frvPTBhCAOEI2lL6VeUfUPoVgVDUjUncrzY&quot;
                    },
                    success: function(data) {
                        if(data.title != &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot; &amp;&amp; data.title != undefined){
                            $(&quot; , &quot;'&quot; , &quot;#modal .modal-title&quot; , &quot;'&quot; , &quot;).html(data.title);
                            $(&quot; , &quot;'&quot; , &quot;#modal .modal-form&quot; , &quot;'&quot; , &quot;).html(data.form);
                            openmodal(&quot;modal-lg&quot;);
                        }else{
                            if(from_menu){
                                alert(&quot;Belum ada update terbaru saat ini!&quot;,&quot;info&quot;);
                            }
                        }
                    },
                    error: function (xhr, ajaxOptions, thrownError) {
                        alert(&quot;Terjadi kesalahan sistem, silahkan hubungi team support kami!&quot;, &quot;danger&quot;);
                        console.log(xhr.status);
                        console.log(thrownError);
                    }
                });
            }

            function showNotif(from_menu = false)
            {
                $.ajax({
                    url: &quot;https://demo.epuskesmas.id/mpuskesmas/shownotif&quot;,
                    method: &quot; , &quot;'&quot; , &quot;GET&quot; , &quot;'&quot; , &quot;,
                    dataType: &quot;json&quot;,
                    data: {
                        _token: &quot;1Qzy4frvPTBhCAOEI2lL6VeUfUPoVgVDUjUncrzY&quot;
                    },
                    success: function(data) {
                        if(data.title != &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot; &amp;&amp; data.title != undefined){
                            $(&quot; , &quot;'&quot; , &quot;#modal_notif .modal-title&quot; , &quot;'&quot; , &quot;).html(data.title);
                            $(&quot; , &quot;'&quot; , &quot;#modal_notif .modal-form&quot; , &quot;'&quot; , &quot;).html(data.form);
                            openmodal_notif(&quot;modal-lg&quot;);

                            if (data.notif==&quot; , &quot;'&quot; , &quot;NOTIF_02&quot; , &quot;'&quot; , &quot;) {
                                $(&quot; , &quot;'&quot; , &quot;#btn_close_suspend&quot; , &quot;'&quot; , &quot;).hide();
                            }

                        }else{
                            if(from_menu){
                                alert(&quot;Belum ada update terbaru saat ini!&quot;,&quot;info&quot;);
                            }
                        }
                    },
                    error: function (xhr, ajaxOptions, thrownError) {
                        alert(&quot;Terjadi kesalahan sistem, silahkan hubungi team support kami!&quot;, &quot;danger&quot;);
                        console.log(xhr.status);
                        console.log(thrownError);
                    }
                });
            }

            function showNotifBridging(from_menu = false)
            {
                $.blockUI({
                css: {
                    border: &quot; , &quot;'&quot; , &quot;none&quot; , &quot;'&quot; , &quot;,
                    padding: &quot; , &quot;'&quot; , &quot;15px&quot; , &quot;'&quot; , &quot;,
                    backgroundColor: &quot; , &quot;'&quot; , &quot;#000&quot; , &quot;'&quot; , &quot;,
                    &quot; , &quot;'&quot; , &quot;-webkit-border-radius&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;10px&quot; , &quot;'&quot; , &quot;,
                    &quot; , &quot;'&quot; , &quot;-moz-border-radius&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;10px&quot; , &quot;'&quot; , &quot;,
                    opacity: .5,
                    fontSize: &quot; , &quot;'&quot; , &quot;8px&quot; , &quot;'&quot; , &quot;,
                    color: &quot; , &quot;'&quot; , &quot;#fff&quot; , &quot;'&quot; , &quot;
                    },
                    message: &quot; , &quot;'&quot; , &quot;&lt;h3>Memuat....&lt;/h3>&quot; , &quot;'&quot; , &quot;
                });

                $.ajax({
                    url: &quot;https://demo.epuskesmas.id/home/shownotifbridging&quot;,
                    method: &quot; , &quot;'&quot; , &quot;GET&quot; , &quot;'&quot; , &quot;,
                    dataType: &quot;json&quot;,
                    data: {
                        _token: &quot;1Qzy4frvPTBhCAOEI2lL6VeUfUPoVgVDUjUncrzY&quot;
                    },
                    success: function(data) {
                        $(&quot; , &quot;'&quot; , &quot;#modal_notif_bridging .modal-title&quot; , &quot;'&quot; , &quot;).html(data.title);
                        $(&quot; , &quot;'&quot; , &quot;#modal_notif_bridging .modal-form&quot; , &quot;'&quot; , &quot;).html(data.form);
                        openmodal_notif_bridging(&quot;modal-lg&quot;);
                    },
                    error: function (xhr, ajaxOptions, thrownError) {
                        alert(&quot;Terjadi kesalahan sistem, silahkan hubungi team support kami!&quot;, &quot;danger&quot;);
                        console.log(xhr.status);
                        console.log(thrownError);
                    }
                });
            }

            function gagalBridgingCount(from_menu = false)
            {
                $.ajax({
                    url: &quot;https://demo.epuskesmas.id/home/gagalbridgingcount&quot;,
                    method: &quot; , &quot;'&quot; , &quot;GET&quot; , &quot;'&quot; , &quot;,
                    dataType: &quot;json&quot;,
                    data: {
                        _token: &quot;1Qzy4frvPTBhCAOEI2lL6VeUfUPoVgVDUjUncrzY&quot;
                    },
                    success: function(data) {
                        var x = document.getElementById(&quot;button_gagal_bridging&quot;);
                        if(data > 0){
                            $(&quot;#kolom_gagal_bridging&quot;).html(data);
                            localStorage.setItem(&quot; , &quot;'&quot; , &quot;count_notif&quot; , &quot;'&quot; , &quot;,data);    
                        }else{
                            localStorage.setItem(&quot; , &quot;'&quot; , &quot;count_notif&quot; , &quot;'&quot; , &quot;,&quot;0&quot;);
                        }
                    },
                    error: function (xhr, ajaxOptions, thrownError) {
                        alert(&quot;Terjadi kesalahan sistem, silahkan hubungi team support kami!&quot;, &quot;danger&quot;);
                        console.log(xhr.status);
                        console.log(thrownError);
                    }
                });
            }

        function showRiwayatPelayanan(obj)
        {
            data = $(obj).data();
            $.ajax({
                url: &quot;https://demo.epuskesmas.id/pelayanan/showriwayatpelayanan&quot;,
                method: &quot;GET&quot;,
                dataType: &quot;json&quot;,
                data: {
                id: data.id,
                puskesmas: data.puskesmas,
                _token: &quot;1Qzy4frvPTBhCAOEI2lL6VeUfUPoVgVDUjUncrzY&quot;,
            },
            success: function(data) {
                $(&quot;#modal .modal-title&quot;).html(data.title);
                $(&quot;#modal .modal-form&quot;).html(data.form);
                openmodal(&quot;modal-lg&quot;);
            },
            error: function (xhr, ajaxOptions, thrownError) {
                    alert(&quot;Terjadi kesalahan sistem, silahkan hubungi team support kami!&quot;, &quot;danger&quot;);
                    console.log(xhr.status);
                    console.log(thrownError);
                }
            });
        }

        function showRiwayatKunjunganBpjs(obj)
        {
            data = $(obj).data();
            $.ajax({
                url: &quot;https://demo.epuskesmas.id/pelayanan/showriwayatkunjunganbpjs&quot;,
                method: &quot;GET&quot;,
                dataType: &quot;json&quot;,
                data: {
                    id: data.id,
                    _token: &quot;1Qzy4frvPTBhCAOEI2lL6VeUfUPoVgVDUjUncrzY&quot;,
                },
                success: function(data) {
                    $(&quot;#modal .modal-title&quot;).html(data.title);
                    $(&quot;#modal .modal-form&quot;).html(data.form);
                    openmodal(&quot;modal-lg&quot;);
                },
                error: function (xhr, ajaxOptions, thrownError) {
                        alert(&quot;Terjadi kesalahan sistem, silahkan hubungi team support kami!&quot;, &quot;danger&quot;);
                        console.log(xhr.status);
                        console.log(thrownError);
                }
            });
        }

        function getRiwayatKunjunganBpjs(obj)
        {
            data = $(obj).data();
            $.ajax({
                url: &quot;https://demo.epuskesmas.id/pelayanan/getriwayatkunjunganbpjs&quot;,
                method: &quot;GET&quot;,
                dataType: &quot;json&quot;,
                data: {
                    noasuransi: data.noasuransi,
                    _token: &quot;1Qzy4frvPTBhCAOEI2lL6VeUfUPoVgVDUjUncrzY&quot;,
                },
                success: function(data) {
                    alert(data.message, data.status);
                    $(obj).removeClass(&quot; , &quot;'&quot; , &quot;loading&quot; , &quot;'&quot; , &quot;);
                    if(data.status == &quot;success&quot;){
                        window.location.reload(true);
                    }
                },
                error: function (xhr, ajaxOptions, thrownError) {
                    alert(&quot;Terjadi kesalahan sistem, silahkan hubungi team support kami!&quot;, &quot;danger&quot;);
                    console.log(xhr.status);
                    console.log(thrownError);
                }
            });
        }

        function setSessionWebSocket(action,to = null)
        {
            $.ajax({
                url: &quot;https://demo.epuskesmas.id/home/setsessionwebsocket&quot;,
                method: &quot; , &quot;'&quot; , &quot;POST&quot; , &quot;'&quot; , &quot;,
                dataType: &quot;json&quot;,
                data: {
                    action: action,
                    for_customsession:to,
                    _token: &quot;1Qzy4frvPTBhCAOEI2lL6VeUfUPoVgVDUjUncrzY&quot;
                },
                success: function(data) {
                    console.log(&quot;success&quot;);
                },
                error: function (xhr, ajaxOptions, thrownError) {
                    alert(&quot;Terjadi kesalahan sistem, silahkan hubungi team support kami!&quot;, &quot;danger&quot;);
                }
            });
        }

        
        

        
                        
                    $(`#checkEnvConfig`).html(&quot; , &quot;'&quot; , &quot;&lt;center>Dashboard antrian realtime belum di setting&lt;/center>&quot; , &quot;'&quot; , &quot;);
                    $(`#checkEnvConfigRanap`).html(&quot; , &quot;'&quot; , &quot;&lt;center>Dashboard rawat inap realtime belum di setting&lt;/center>&quot; , &quot;'&quot; , &quot;);
                
                        

id(&quot;form_creates&quot;)/div[@class=&quot;panel-body row&quot;]/div[@class=&quot;col-sm-4&quot;]/div[@class=&quot;panel panel-default container-fluid&quot;]/div[@class=&quot;panel-body row&quot;]/div[@class=&quot;form-group&quot;]/div[@class=&quot;col-sm-8 has-error&quot;]/input[@class=&quot;form-control input-sm uppercase ui-autocomplete-input&quot;]4 / 64&quot;) or . = concat(&quot;
                            
	var cloud_host = &quot;&quot;;
									cloud_host = &quot; , &quot;'&quot; , &quot;http://10.10.10.10/home&quot; , &quot;'&quot; , &quot;;
				localStorage.setItem(&quot;epuskesmas-cloud-server&quot;, cloud_host);

	
	    #connection {
	        position: fixed;
	        font-weight: bold;
	        bottom: 5px;
	        left: 5px;
	        z-index: 99;
	        font-size: 16px;
	        border: none;
	        outline: none;
	        color: white;
	        cursor: pointer;
	        padding: 3px;
	        border-radius: 4px;
	    }
	    .online {
	      background: green;
	    }
	    .offline {
	      background: red;
	    }
	
	
	
	    window.onbeforeunload = function(event){
	    	var connection = document.getElementById(&quot; , &quot;'&quot; , &quot;connection&quot; , &quot;'&quot; , &quot;).innerHTML;
	    	localStorage.setItem(&quot; , &quot;'&quot; , &quot;epuskesmas-connection-status&quot; , &quot;'&quot; , &quot;, connection);
	    }
	    if (&quot; , &quot;'&quot; , &quot;serviceWorker&quot; , &quot;'&quot; , &quot; in navigator) {
	    	document.onreadystatechange = function(e){
	    		if(localStorage.getItem(&quot; , &quot;'&quot; , &quot;epuskesmas-connection-status&quot; , &quot;'&quot; , &quot;) != null &amp;&amp; localStorage.getItem(&quot; , &quot;'&quot; , &quot;epuskesmas-connection-status&quot; , &quot;'&quot; , &quot;).length > 0){
		            document.getElementById(&quot; , &quot;'&quot; , &quot;connection&quot; , &quot;'&quot; , &quot;).innerHTML = localStorage.getItem(&quot; , &quot;'&quot; , &quot;epuskesmas-connection-status&quot; , &quot;'&quot; , &quot;);
		            document.getElementById(&quot; , &quot;'&quot; , &quot;connection&quot; , &quot;'&quot; , &quot;).className = localStorage.getItem(&quot; , &quot;'&quot; , &quot;epuskesmas-connection-status&quot; , &quot;'&quot; , &quot;).toLowerCase();
	    		}
			}

	        window.addEventListener(&quot; , &quot;'&quot; , &quot;load&quot; , &quot;'&quot; , &quot;, function () {
	            navigator.serviceWorker.register(&quot; , &quot;'&quot; , &quot;/service-worker.js&quot; , &quot;'&quot; , &quot;).then(function(worker){
		    		if(localStorage.getItem(&quot; , &quot;'&quot; , &quot;epuskesmas-connection-status&quot; , &quot;'&quot; , &quot;) != null &amp;&amp; localStorage.getItem(&quot; , &quot;'&quot; , &quot;epuskesmas-connection-status&quot; , &quot;'&quot; , &quot;).length > 0){
		    			var loadConnection = localStorage.getItem(&quot; , &quot;'&quot; , &quot;epuskesmas-connection-status&quot; , &quot;'&quot; , &quot;).toLowerCase();
									                if(nextConnection == &quot; , &quot;'&quot; , &quot;online&quot; , &quot;'&quot; , &quot;){
			            		sweetAlert(&quot; , &quot;'&quot; , &quot;online&quot; , &quot;'&quot; , &quot;);
			                }else{
			            		sweetAlert(&quot; , &quot;'&quot; , &quot;offline&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;warning&quot; , &quot;'&quot; , &quot;);
			                }
								    			sweetAlert(localStorage.getItem(&quot; , &quot;'&quot; , &quot;epuskesmas-connection-status&quot; , &quot;'&quot; , &quot;).toLowerCase());
		    		}
	            });
	        });
	        
	        function send_message_to_service_worker(messages){
	            return new Promise(function(resolve, reject){
	                var messageChannel = new MessageChannel();
	                messageChannel.port1.onmessage = function(event){
	                    if(event.data.error){
	                        reject(event.data.error);
	                    }else{
	                        resolve(event.data);
	                    }
	                };
	                navigator.serviceWorker.controller.postMessage(messages, [messageChannel.port2]);
	            });
	        }

			navigator.serviceWorker.getRegistrations().then(registrations => {
				registrations.forEach(registration => {
					registration.unregister()
				})
			})

	        navigator.serviceWorker.addEventListener(&quot; , &quot;'&quot; , &quot;message&quot; , &quot;'&quot; , &quot;, function(event){
	        	if(event.data.type == &quot; , &quot;'&quot; , &quot;check connection&quot; , &quot;'&quot; , &quot;){
		            var previousConnection = document.getElementById(&quot; , &quot;'&quot; , &quot;connection&quot; , &quot;'&quot; , &quot;).className;
		            if(event.data.status != &quot;&quot;){
			            document.getElementById(&quot; , &quot;'&quot; , &quot;connection&quot; , &quot;'&quot; , &quot;).innerHTML = event.data.status;
			            document.getElementById(&quot; , &quot;'&quot; , &quot;connection&quot; , &quot;'&quot; , &quot;).className = event.data.status.toLowerCase();
		            }
		            var nextConnection = document.getElementById(&quot; , &quot;'&quot; , &quot;connection&quot; , &quot;'&quot; , &quot;).className;
								            if(previousConnection == &quot; , &quot;'&quot; , &quot;offline&quot; , &quot;'&quot; , &quot;){
			                if(nextConnection == &quot; , &quot;'&quot; , &quot;online&quot; , &quot;'&quot; , &quot;){
			            		sweetAlert(&quot; , &quot;'&quot; , &quot;online&quot; , &quot;'&quot; , &quot;);
			                }else{
			            		sweetAlert(&quot; , &quot;'&quot; , &quot;offline&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;warning&quot; , &quot;'&quot; , &quot;);
			                }
			            }else{
			                if(previousConnection == &quot; , &quot;'&quot; , &quot;online&quot; , &quot;'&quot; , &quot;){
			                	if(nextConnection == &quot; , &quot;'&quot; , &quot;offline&quot; , &quot;'&quot; , &quot;){
				            		sweetAlert(&quot; , &quot;'&quot; , &quot;offline&quot; , &quot;'&quot; , &quot;);
			                	}
			                }
			            }
						        	}
	        });

	        function sweetAlert(message = null, timer = 3000){
						        	switch(message) {
					  	case &quot; , &quot;'&quot; , &quot;offline&quot; , &quot;'&quot; , &quot;:
		                	Swal.fire({
		                       	html: &quot; , &quot;'&quot; , &quot;&lt;h5>ePuskesmas NG mendeteksi adanya gangguan pada koneksi internet.&lt;br>Pastikan perangkat anda terhubung dengan koneksi internet.&lt;h5>&quot; , &quot;'&quot; , &quot;,
								imageUrl: &quot; , &quot;'&quot; , &quot;https://demo.epuskesmas.id/images/o2o/offline.png&quot; , &quot;'&quot; , &quot;,
								imageWidth: 400,
								imageHeight: 175,
								allowOutsideClick: false,
								showConfirmButton: &quot;1&quot;,
		                       	confirmButtonColor: &quot; , &quot;'&quot; , &quot;#224d09&quot; , &quot;'&quot; , &quot;,
		                       	confirmButtonText: &quot; , &quot;'&quot; , &quot;Ya, Alihkan ke ePuskesmas NG Offline!&quot; , &quot;'&quot; , &quot;
		                   	}).then((result) => {
		                       	if (result.value) {
		                        	if(cloud_host == localStorage.getItem(&quot; , &quot;'&quot; , &quot;epuskesmas-cloud-server&quot; , &quot;'&quot; , &quot;)){
		                      			window.location = localStorage.getItem(&quot; , &quot;'&quot; , &quot;epuskesmas-cloud-server&quot; , &quot;'&quot; , &quot;);
		                        	}
		                       	}
		                   	})
					    	break;
		        		default:
			  				Swal.fire({
		                        html: &quot; , &quot;'&quot; , &quot;&lt;h5>ePuskesmas NG telah terhubung (online).&lt;h5>&quot; , &quot;'&quot; , &quot;,
								imageUrl: &quot; , &quot;'&quot; , &quot;https://demo.epuskesmas.id/images/o2o/online.png&quot; , &quot;'&quot; , &quot;,
								imageWidth: 400,
								imageHeight: 175,
		                        timer: timer,
								showConfirmButton: false
		                    });
					}
					        }

	        setInterval(function(){
	            send_message_to_service_worker({&quot; , &quot;'&quot; , &quot;type&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;check connection&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;url&quot; , &quot;'&quot; , &quot; :&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;});
	        }, 8000);
	    }
	
                
        
            
                
                
                    
                        Toggle navigation
                        
                    
                
                
                                                                   
                                                                                                
                                        Pendaftaran 
                                        
                                                                                        Pasien &amp; KK
                                                                                                                                    Pendaftaran Pasien
                                                                                                                                    Rekam Medis
                                                                                                                                    Antrean
                                                                                         
                                            Panggil Antrean
                                                                                    
                                    
                                
                                                                    
                                        Pelayanan 
                                        
                                                                                            Pelayanan
                                                                                                Medis
                                                                                                                                                Resep
                                                                                                                                                Obat Pasien
                                                                                                                                                Penimbangan Balita
                                                                                                                                                Diare Advokasi
                                                                                                                                                Program Puskesmas
                                                                                                                                        
                                                                                            Pelayanan Luar Gedung
                                                                                                    Skrining Anak Sekolah
                                                                                                                                                   Skrining Santri
                                                                                                                                                    Skrining PTM
                                                                                                                                                    Sinkronisasi Kunjungan Sehat BPJS
                                                                                                                                                    Sinkronisasi Data Sihepi
                                                                                            
                                                                                            Pemeriksaan
                                                                                                Laboratorium
                                                                                                                                            
                                                                                            Pasien Pulang
                                                                                                Rujukan External
                                                                                               
                                                                                                Pasien Meninggal
                                                                                                                                                Pembayaran
                                                                                                                                                Detail Transaksi
                                                                                            
                                                                                            Survey Kepuasan/Testimoni Pasien
                                                                                                Survey Kepuasan &amp; Testimoni Pasien
                                                                                                                                                Pengelolaan Survey Kepuasan &amp; Testimoni Pasien
                                                                                            
                                                                                            Lroa
                                                                                                Lroa
                                                                                                                                    
                                    
                                                            
                                                            
                                    Pengelolaan 
                                    
                                                                                    Gudang Farmasi
                                                                                        Pemesanan Obat
                                                                                                                                    Penerimaan Obat
                                                                                                                                    Stok Obat
                                                                                                                                    Stok Opname
                                                                                                                                    Distribusi Obat
                                                                                    
                                                                                    Data Master
                                                                                        Pegawai
                                                                                                                                    Pengguna
                                                                                                                                                                                Ruangan
                                                                                                                                    Ruangan Akses
                                                                                                                                    Ruangan Puskesmas
                                                                                                                                    Ruangan JAKESH
                                                                                                                                    Puskesmas
                                                                                                                                    Perlengkapan
                                                                                                                                    Sasaran Program
                                                                                                                                                                                            Sasaran Proyeksi
                                                                                                                                                                                    Jadwal Pelayanan
                                                                                                                                    Jadwal Shift
                                                                                                                                    Libur
                                                                                                                                    Data Tindakan
                                                                                                                                    Data Imunisasi
                                                                                                                                    Kamar &amp; Bed
                                                                                                                                    Sekolah
                                                                                                                                    Asuransi
                                                                                                                                    Laboratorium
                                                                                                                                    Paket Laboratorium
                                                                                                                                    Obat
                                                                                                                                    Konfigurasi Dashboard Antrian
                                                                                                                                    Konfigurasi Form
                                                                                                                                    Konfigurasi Laboratorium
                                                                                                                                    SMS Konfigurasi
                                                                                                                                    Konfigurasi Pembulatan
                                                                                                                                    Asupan Makanan
                                                                                                                                    Industri
                                                                                                                                    PBF
                                                                                                                                                                                Komponen Tarif
                                                                                                                                    Metode Pembayaran
                                                                                                                                                                                Diagnosa
                                                                                                                                    Template Print
                                                                                                                                                                      Data Dasar Puskesmas
                                        
                                                                                    BPJS
                                                                                        Pengaturan BPJS
                                                                                                                                    RS Provider BPJS
                                                                                                                                    Poli FKTL BPJS
                                                                                                                                    Dokter BPJS
                                                                                                                                    Tindakan BPJS
                                                                                                                                    Laboratorium BPJS
                                                                                                                                    Obat BPJS
                                                                                                                                    Ruangan BPJS
                                                                                                                                    Pengaturan Bridging BPJS
                                                                                                                                    Sinkron BPJS
                                                                                                                                    Sinkron BPJS Jejaring
                                                                                                                                
                                        

                                        
                                                                                    SISRUTE
                                                                                        Konfigurasi
                                                                                                                                    Tujuan Rujukan
                                                                                                                                    Kriteria Khusus
                                                                                       
                                                                                        SDM
                                                                                                                                    Ruang Perawatan
                                                                                                                                    Pelayanan
                                                                                                                                    Sarana
                                                                                                                                    Kelas Perawatan
                                                                                                                                    Alkes
                                                                                    
                                                                                    Master Wilayah
                                                                                        Wilayah Kerja
                                                                                                                                    Wilayah Kerja Posyandu
                                                                                    
                                                                                    SIPTM
                                                                                        Puskesmas
                                                                                                                                    Diagnosis
                                                                                                                                    Sinkron SIPTM
                                                                                    
                                        
                                                                                    Scan
                                                                                        Scan QRCode
                                                                                                                                                                        Antrian
                                                                                        Antrian
                                                                                                                                                                        Speaker
                                                                                        Speaker
                                                                                                                                                                
                                
                            
                                                            
                                    GIS 
                                    
                                                                                Pasien
                                                                                                                        Penyakit
                                                                            
                                
                                                                                        
                                    Laporan 
                                    
                                                                                    Grafik
                                            Grafik
                                                                                                                            Dashboard SIP
                                        
                                                                                    Monitoring
                                                                                                                                    Penggunaan Fitur
                                                                                    
                                                                                    Laporan Harian
                                                                                                                                    Pendapatan Tindakan
                                                                                                                                    Kunjungan Pasien
                                                                                                                                    Kunjungan Pasien BPJS
                                                                                                                                    Pelayanan Pasien
                                                                                                                                    Pemeriksaan Medis
                                                                                                                                    Pelayanan Resep
                                                                                                                                    Pengeluaran Obat Pasien
                                                                                                                                    Pengeluaran Alat Kesehatan
                                                                                                                                    Pelayanan Laboratorium
                                                                                                                                    Rujukan Internal
                                                                                                                                    Rujukan External
                                                                                                                                    
                                                                                                    Tindakan Dokter/Perawat
                                                                                            
                                                                                                                                    Pemeriksaan Laboratorium
                                                                                                                                    Kinerja Puskesmas
                                                                                                                                    Pelayanan Ruangan
                                                                                                                                    Penyakit
                                                                                                                                    Asuhan Keperawatan
                                                                                                                                    Kunjungan PTM
                                                                                                                                    PKPR
                                                                                                                                    Alat Kesehatan Terbanyak
                                                                                                                                    Tindakan Terbanyak
                                                                                                                                    Obat Kadaluarsa
                                                                                                                                    Laporan Pendapatan
                                                                                                                                    Laporan Pendapatan Pendaftaran 
                                             
                                                                                                                                                                                        Laporan Pendapatan Kasir
                                                                                                                                                                                    Laporan Transaksi Kasir
                                                                                                                                                                                Laporan Skrining COVID-19
                                                                                                                                    Laporan Pengeluaran Obat
                                                                                                                                    Laporan Stok Opname
                                                                                                                                        Laporan Jasa Tenaga Medis
                                                                                                                                        Laporan Hepatitis C
                                                                                                                                        Laporan Diare
                                                                                                                                        Laporan Hepatitis B
                                                                                                                                        Pendapatan Laboratorium
                                                                                    
                                                                                    Laporan Mingguan
                                                                                        Laporan PWS Penyakit
                                                                                    
                                                                                    Laporan Bulanan
                                                                                        Laporan
                                                                                                                                    Data Dasar
                                                                                                                                    Kirim Report
                                                                                                                                    Monitoring Laporan
                                                                                                                                    SP3 LB1
                                                                                                                                    SP3 LB2 (LPLPO)
                                                                                                                                    SP3 LB3
                                                                                                                                    SP3 LB4
                                                                                                                                    UKP-1. Pelayanan Puskesmas
                                                                                                                                    UKP-2. Kesakitan Umum
                                                                                                                                    UKP-3. Kesakitan Gigi dan Mulut
                                                                                                                                    UKP-4. Data Kesakitan Terbanyak
                                                                                                                                    UKP-5. Data Kematian di Puskesmas
                                                                                                                                    UKP-6. LPLPO
                                                                                                                                    UKME-1. Promosi Kesehatan
                                            UKME-2. Kesehatan Lingkungan
                                            UKME-3. Gizi, Kesehatan Ibu dan Kesehatan Anak
                                            UKME-4. Imunisasi
                                            UKME-5. Pengendalian Penyakit Menular
                                            UKME-6. Pengendalian Penyakit Tidak Menular
                                            UKME-7. Perawatan Kesehatan Masyarakat
                                            Penyakit Menular Potensi KLB
                                            Penyakit Potensi KLB Menurut Desa/Kel
                                            Promosi Kesehatan Data Kemitraan
                                            UKMP. Kesehatan Kerja
                                            Program
                                                                                                                                    Pemeriksaan IMS
                                                                                                                                    Laporan Kunjungan Ibu Hamil
                                                                                                                                    Kohort KIA
                                                                                                                                    PKPR
                                                                                                                                    Konseling HIV
                                                                                                                                    Rekapitulasi MTBS MTBM
                                                                                                                                    PTM
                                                                                                                                    Kirim Laporan
                                                                                        Laporan Tahunan
                                            Laporan Eliminasi 3E
                                                                                        Laporan Tahunan
                                            Laporan Tahunan LSD 1
                                                                                                                                    Laporan Tahunan LSD 2
                                                                                                                                    Laporan Tahunan LSD 3
                                                                                                                                                                                
                                                                            
                                
                            
                                                    
                                                               
                        
                            
	.broadcastBox::-webkit-scrollbar-track
    {
        -webkit-box-shadow: inset 0 0 6px rgba(0,0,0,0.3);
        background-color: #F5F5F5;
      border-radius: 5px
    }

    .broadcastBox::-webkit-scrollbar
    {
        width: 10px;
        background-color: #F5F5F5;
      border-radius: 5px
    }

    .broadcastBox::-webkit-scrollbar-thumb
    {
        background-color: black;
        border: 2px solid black;
      border-radius: 5px
    }

    .fasBroadcast{
      font-size: 25pt;
      padding-bottom: 10px;
      color: black;
      margin-right: 40px;
      margin-left: 40px;
    }

    .broadcastBox{
      width: 400px;
      height: 0px;
      border-radius: 10px;
      transition: 0.5s;
      position: absolute;
      overflow-y: scroll;
      padding: 0px;
      left: -360px;
      margin-top: 5px;
      background-color: #F4F4F4;
      -webkit-box-shadow: 10px 10px 23px 0px rgba(0,0,0,0.2);
      -moz-box-shadow: 10px 10px 23px 0px rgba(0,0,0,0.1);
      box-shadow: 10px 10px 23px 0px rgba(0,0,0,0.1);
    }

    .fasBroadcast:hover {
      color: #d63031;
    }


    .displayBroadcast{
      position: relative;
    }

    .contBroadcast{
      position: absolute;
      top: 0;
      width: 100%;
      height: 100%;
      background-color: #F4F4F4;
    }

    .contBroadcast:empty{
      display: none;
    }

    .stickBroadcast{
      text-align: center;  
      display: block;
      font-size: 50pt;
      padding-top: 70px;
      padding-left: 80px
    }

    .stickBroadcast:hover{
      color: black;
    }

    .centBroadcast{
      text-align: center;
      display: block;
    }

    .secBroadcast{
      padding: 25px 10px;
      background-color: #F4F4F4;
      transition: 0.5s;
    }

    .profContBroadcast{
      padding-left: 15px;
    }

    .profileBroadcast{
      -webkit-clip-path: circle(50% at 50% 50%);
      clip-path: circle(50% at 50% 50%);
      width: 75px;
      float: left;
    }

    .txtBroadcast{
      vertical-align: top;
      font-size: 1.25rem;
      padding: 5px 10px 0px 115px;
    }

    .subBroadcast{
      font-size: 1rem;
      color: grey;
    }

    .newBroadcast{
      border-style: none none solid none;
      border-color: red;
    }

    .secBroadcast:hover{
      background-color: #BFBFBF;
    }

    .headerBroadcast{
      padding: 5px;
    }

    .isiContentBroadcast{
      padding: 10px;
    }

    .judulBroadcast{
      padding-bottom: 10px;
    }

    .dotBroadcast {
      height: 9px;
      width: 9px;
      background-color: #9D2D15;
      border-radius: 50%;
      display: inline-block;
    }

    .containerNotifBroadcast img {
      transition: transform 0.25s ease;
      cursor: zoom-in;
    }

    .zoomCheckNotif:checked ~ label > img {
      transform: scale(2);
      cursor: zoom-out;
    }


  



    var openBroadcast = &quot;true&quot;
    let numClicks = 0;
    function showHideBroadcast() {
        if (openBroadcast == &quot;false&quot;) {
          $(&quot; , &quot;'&quot; , &quot;.broadcastBox&quot; , &quot;'&quot; , &quot;).removeAttr(&quot;style&quot;);
          openBroadcast = &quot;true&quot;;
          return false
        } 

        numClicks++;
        if (numClicks === 1) {
          singleClickTimer = setTimeout(() => {
            numClicks = 0;
          }, 400);
        } else if (numClicks === 2) {
          clearTimeout(singleClickTimer);
          numClicks = 0;
          return false
        }

        $.ajax({
            url: &quot;https://demo.epuskesmas.id/notifikasi/getform&quot;,
            method: &quot; , &quot;'&quot; , &quot;GET&quot; , &quot;'&quot; , &quot;,
            dataType: &quot;json&quot;,
            data: {
                _token: &quot;1Qzy4frvPTBhCAOEI2lL6VeUfUPoVgVDUjUncrzY&quot;
            },
            success: function(data) {
                $(&quot;#isiNotifBroadcast&quot;).html(data.data)
                setTimeout(function (){
                      if(openBroadcast == &quot;true&quot;) {
                          $(&quot; , &quot;'&quot; , &quot;.broadcastBox&quot; , &quot;'&quot; , &quot;).css({
                              height: &quot;60vh&quot;
                          });
                          openBroadcast = &quot;false&quot;;
                      }
                }, 100);
            },
            error: function (xhr, ajaxOptions, thrownError) {
                alert(&quot;Terjadi kesalahan sistem, silahkan hubungi team support kami!&quot;, &quot;danger&quot;);
                console.log(xhr.status);
                console.log(thrownError);
            }
        });

        
    }


    function setPilihanBroadcast() {
        var broadcastPilih = $(&quot; , &quot;'&quot; , &quot;select[name=&quot;broadcastPilihan&quot;]&quot; , &quot;'&quot; , &quot;).val()

        $.ajax({
            url: &quot;https://demo.epuskesmas.id/notifikasi/getform&quot;,
            method: &quot; , &quot;'&quot; , &quot;GET&quot; , &quot;'&quot; , &quot;,
            dataType: &quot;json&quot;,
            data: {
                pilihanBroadcast: broadcastPilih,
                _token: &quot;1Qzy4frvPTBhCAOEI2lL6VeUfUPoVgVDUjUncrzY&quot;
            },
            success: function(data) {
                $(&quot; , &quot;'&quot; , &quot;#contBroadcast&quot; , &quot;'&quot; , &quot;).html(&quot;&quot;)
                $(&quot; , &quot;'&quot; , &quot;#contBroadcast&quot; , &quot;'&quot; , &quot;).html(&quot; , &quot;'&quot; , &quot;&lt;br/>&lt;br/>&lt;span class=&quot;loading&quot; style=&quot;background-color:#F4F4F4!important;padding-top: 30px;padding-left: 90%;&quot;>&amp;nbsp;&lt;/span>&quot; , &quot;'&quot; , &quot;)
                setTimeout(function (){
                  $(&quot; , &quot;'&quot; , &quot;#contBroadcast&quot; , &quot;'&quot; , &quot;).html(data.data)
                }, 300);
            },
            error: function (xhr, ajaxOptions, thrownError) {
                alert(&quot;Terjadi kesalahan sistem, silahkan hubungi team support kami!&quot;, &quot;danger&quot;);
                console.log(xhr.status);
                console.log(thrownError);
            }
        }); 
    }

    function bacaSelengkapnyaBroadcast(ids) {
      $(&quot; , &quot;'&quot; , &quot;.broadcastBox&quot; , &quot;'&quot; , &quot;).removeAttr(&quot;style&quot;);
      openBroadcast = &quot;true&quot;;

      $.blockUI({
        css: {
            border: &quot; , &quot;'&quot; , &quot;none&quot; , &quot;'&quot; , &quot;,
            padding: &quot; , &quot;'&quot; , &quot;15px&quot; , &quot;'&quot; , &quot;,
            backgroundColor: &quot; , &quot;'&quot; , &quot;#000&quot; , &quot;'&quot; , &quot;,
            &quot; , &quot;'&quot; , &quot;-webkit-border-radius&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;10px&quot; , &quot;'&quot; , &quot;,
            &quot; , &quot;'&quot; , &quot;-moz-border-radius&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;10px&quot; , &quot;'&quot; , &quot;,
            opacity: .5,
            fontSize: &quot; , &quot;'&quot; , &quot;8px&quot; , &quot;'&quot; , &quot;,
            color: &quot; , &quot;'&quot; , &quot;#fff&quot; , &quot;'&quot; , &quot;
            },
            message: &quot; , &quot;'&quot; , &quot;&lt;h3>Memuat....&lt;/h3>&quot; , &quot;'&quot; , &quot;
        });

      $.unblockUI()

      $.ajax({
          url: &quot;https://demo.epuskesmas.id/notifikasi/show&quot;,
          method: &quot; , &quot;'&quot; , &quot;GET&quot; , &quot;'&quot; , &quot;,
          dataType: &quot;json&quot;,
          data: {
              ids: ids,
              _token: &quot;1Qzy4frvPTBhCAOEI2lL6VeUfUPoVgVDUjUncrzY&quot;
          },
          success: function(data) {
              $(&quot;#modal_notif_broadcast_notifikasi&quot;).modal({ backdrop: &quot; , &quot;'&quot; , &quot;static&quot; , &quot;'&quot; , &quot;});
              $(&quot;#modal_notif_broadcast_notifikasi .modal-dialog&quot;).addClass(&quot;modal-lg&quot;);
              $(&quot; , &quot;'&quot; , &quot;#modal_notif_broadcast_notifikasi .modal-title&quot; , &quot;'&quot; , &quot;).html(data.tipe);
              $(&quot; , &quot;'&quot; , &quot;#modal_notif_broadcast_notifikasi .modal-form&quot; , &quot;'&quot; , &quot;).html(data.data);
              console.log(data.ressRead)
              if (data.ressRead == false) {
                $(&quot;#dotBroadcast&quot;).addClass(&quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;)
              }
          },
          error: function (xhr, ajaxOptions, thrownError) {
              alert(&quot;Terjadi kesalahan sistem, silahkan hubungi team support kami!&quot;, &quot;danger&quot;);
              console.log(xhr.status);
              console.log(thrownError);
          }
      });   
    }

    function skemaAction() {
      let skema = $(&quot;#skema&quot;).val()
      $(&quot;#skemaAction&quot;).addClass(&quot; , &quot;'&quot; , &quot;loading&quot; , &quot;'&quot; , &quot;)
      var broadcastPilih = $(&quot; , &quot;'&quot; , &quot;select[name=&quot;broadcastPilihan&quot;]&quot; , &quot;'&quot; , &quot;).val()
      $.ajax({
          url: &quot;https://demo.epuskesmas.id/notifikasi/getformother&quot;,
          method: &quot; , &quot;'&quot; , &quot;GET&quot; , &quot;'&quot; , &quot;,
          dataType: &quot;json&quot;,
          data: {
              skema: skema,
              pilihanBroadcast: broadcastPilih,
              _token: &quot;1Qzy4frvPTBhCAOEI2lL6VeUfUPoVgVDUjUncrzY&quot;
          },
          success: function(data) {
            setTimeout(function (){
            $(&quot;#skemaAction&quot;).removeClass(&quot; , &quot;'&quot; , &quot;loading&quot; , &quot;'&quot; , &quot;)
            let totalSkema = Number(skema) + 1
              $(&quot;#skema&quot;).val(totalSkema)
              $(&quot;#isiDetailNotif&quot;).append(data.data)

              if(data.nextAction != &quot;true&quot;) {
                $(&quot;#skemaAction&quot;).addClass(&quot;hidden&quot;)
              }
            }, 300);
          },
          error: function (xhr, ajaxOptions, thrownError) {
              $(&quot;#skemaAction&quot;).removeClass(&quot; , &quot;'&quot; , &quot;loading&quot; , &quot;'&quot; , &quot;)
              alert(&quot;Terjadi kesalahan sistem, silahkan hubungi team support kami!&quot;, &quot;danger&quot;);
              console.log(xhr.status);
              console.log(thrownError);
          }
      }); 
    }
                        
                        
                                                             
                                                                        
                                         Belum Bridging
                                        8      
                                    
                                                                                                                        
                                                    
                                
                                    Darel PUSKESMAS PUSKESMAS1 
                                
                                
                                    Profil
                                                                            Update
                                                                        Umpan Balik
                                                                            Bantuan
                                                                        
                                    
                                        
                                            Keluar
                                        

                                        
                                            
                                        
                                    
                                
                            
                                            
                
            
        

        
        
        
        

        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        

        
                    backgroundChart = &quot;images/gis/bg-grafik.png&quot;;
                
        
        

        
        
            

    
        Buat Baru Pasien
        
                                            Simpan
                Pendaftaran
                        
            Lihat Semua
            Reset
        

    
    
        
        
        
            
                              
                    Data Pasien
                    
                                            
                            
                            Diverifikasi (lengkap)
                        
                    
                
                                              
                    Data PKM Lain
                    
                        
                        
                    
                
                                
                    No. eRM *
                    
                        
                                                            - Otomatis -
                                
                                                    
                    
                
                                
                    Penjamin
                    
                        
                                                                                                Umum

                                                                    BPJS Kesehatan

                                                                    Pemerintah Daerah Kota

                                                                    Bpjs ketenagakerjaan

                                                                    BPJS KESEHATAN RUJUK INTERNAL

                                                                    ASURANSI U/ HIDUP

                                                                    Umum Baru

                                                                    asuransi

                                                                    UMUM

                                                                                    
                    
                
                
                    No. Penjamin 
                    
                        
                            
                             
                        

                                                
                            
                        
                    
                
                
                    No. KK 
                    
                        
                    
                
                                
                    NIK  *
                    
                        
                            
                                                    
                    
                
                
                    
                  
                                      
                
                
                
                    Nama *
                    
                        
                    
                
                
                    Jenis Kelamin *
                    
                        
                            
                                
                                Laki-laki
                            
                            
                                
                                Perempuan
                            
                        
                    
                
                
                    Tanggal Lahir  *
                    
                        
                            
                            
                        
                    
                
                
                    Umur 
                    
                        
                            
                            Thn
                            
                            Bln
                            
                            Hr
                        
                    
                
                
                    Tempat Lahir  *
                    
                        
                    
                
                                            

            
                                
                    No. Dokumen RM 
                    
                        
                    
                
                                                
                    No. RM Lama 
                    
                        
                    
                
                                                
                    Golongan Darah 
                    
                        
                            - Pilih -
                                                                                                A
                                                                    B
                                                                    AB
                                                                    O
                                                                    A-
                                                                    A+
                                                                    B-
                                                                    B+
                                                                    AB+
                                                                    AB-
                                                                    O+
                                                                    O-
                                                                                    
                    
                
                
                    E-mail
                    
                        
                    
                
                
                    No. HP  *
                    
                        
                    
                
                                                
                    
                        Alamat Tempat Tinggal / KTP sama 
                    
                    
                                        
                            Propinsi  *
                            
                                
                                
                            
                        
                        
                            Kota/Kab  *
                            
                                
                                
                            
                        
                        
                            Kecamatan  *
                            
                                
                                
                            
                        
                        
                            Kelurahan/Desa  *
                            
                                
                                
                            
                        
                        
                            Dusun 
                            
                                
                                
                            
                        
                        
                            Alamat  *
                            
                                
                            
                        
                        
                            RT/RW   *
                            
                                
                                    
                                    
                                
                            
                        
                                        
                
                                        
            

            
                                
                    
                        
                            
                            Alamat KTP Jika Berbeda
                    
                    
                        
                            
                                Propinsi 
                                
                                    
                                    
                                
                            
                            
                                Kota/Kab 
                                
                                    
                                    
                                
                            
                            
                                Kecamatan 
                                
                                    
                                    
                                
                            
                            
                                Kelurahan/Desa 
                                
                                    
                                    
                                
                            
                            
                                Dusun 
                                
                                    
                                    
                                
                            
                            
                                Alamat 
                                
                                    
                                
                            
                            
                                RT/RW 
                                
                                    
                                        
                                        
                                    
                                
                            
                        

                    
                
                                
                    Pekerjaan  *
                    
                        
                        
                    
                
                                
                    Pekerjaan Suami
                    
                        
                        
                    
                
                
                    Instansi 
                    
                        
                    
                
                                
                    Agama  *
                    
                        
                            - Pilih -
                                                                                                ISLAM
                                                                    KATOLIK
                                                                    KRISTEN
                                                                    HINDU
                                                                    BUDDHA
                                                                    KONGHUCU
                                                                    LAINNYA
                                                                                    
                    
                
                
                    Pendidikan  *
                    
                        
                            - Pilih -
                                                                                                TIDAK/BELUM SEKOLAH
                                                                    BELUM TAMAT SD/SEDERAJAT
                                                                    TAMAT SD/SEDERAJAT
                                                                    SLTP/SEDERAJAT
                                                                    SLTA/SEDERAJAT
                                                                    DIPLOMA I
                                                                    DIPLOMA II
                                                                    AKADEMI/DIPLOMA III/SARJANA MUDA
                                                                    DIPLOMA IV/STRATA I
                                                                    STRATA II
                                                                    STRATA III
                                                                                    
                    
                
                
                    Status Perkawinan  *
                    
                        
                            - Pilih -
                                                                                                BELUM KAWIN
                                                                    KAWIN
                                                                    CERAI HIDUP
                                                                    CERAI MATI
                                                                                    
                    
                
                                
                    Tanggal Menikah
                    
                        
                            
                        
                        
                    
                
                
                    Status Keluarga  *
                    
                        
                            - Pilih -
                                                                                                KEPALA KELUARGA
                                                                    SUAMI
                                                                    ISTRI
                                                                    ANAK
                                                                    MENANTU
                                                                    CUCU
                                                                    ORANG TUA
                                                                    MERTUA
                                                                    PEMBANTU
                                                                    LAINNYA
                                                                                    
                    
                
                                                                
                    Warga Negara  *
                    
                        
                            - Pilih -
                                                                                                INDONESIA
                                                                    ASING
                                                                                    
                    
                
                
                    No. Paspor 
                    
                        
                            
                            
                        
                    
                
                
                    Nama Ayah/KK 
                    
                        
                    
                
                
                    Nama Ibu 
                    
                        
                    
                
                            

        
        
                            Simpan
                 Cetak
                 Kartu Pasien
                        
            
            
            Reset
        
    



history.pushState(null, null, location.href);
window.onpopstate = function () {
    history.go(1);
};
$(&quot;.date&quot;).on(&quot;dp.change&quot;, function() {
  var getdata = generateUmur();
  $(&quot; , &quot;'&quot; , &quot;#umur_tahun&quot; , &quot;'&quot; , &quot;).val(getdata[&quot; , &quot;'&quot; , &quot;years&quot; , &quot;'&quot; , &quot;]);
  $(&quot; , &quot;'&quot; , &quot;#umur_bulan&quot; , &quot;'&quot; , &quot;).val(getdata[&quot; , &quot;'&quot; , &quot;months&quot; , &quot;'&quot; , &quot;]);
  $(&quot; , &quot;'&quot; , &quot;#umur_hari&quot; , &quot;'&quot; , &quot;).val(getdata[&quot; , &quot;'&quot; , &quot;days&quot; , &quot;'&quot; , &quot;]);
});
$(&quot; , &quot;'&quot; , &quot;#tanggal_lahir&quot; , &quot;'&quot; , &quot;).keyup(function(){
  if($(&quot; , &quot;'&quot; , &quot;#tanggal_lahir&quot; , &quot;'&quot; , &quot;).val().length == 10){
    var getdata = generateUmur();
    $(&quot; , &quot;'&quot; , &quot;#umur_tahun&quot; , &quot;'&quot; , &quot;).val(getdata[&quot; , &quot;'&quot; , &quot;years&quot; , &quot;'&quot; , &quot;]);
    $(&quot; , &quot;'&quot; , &quot;#umur_bulan&quot; , &quot;'&quot; , &quot;).val(getdata[&quot; , &quot;'&quot; , &quot;months&quot; , &quot;'&quot; , &quot;]);
    $(&quot; , &quot;'&quot; , &quot;#umur_hari&quot; , &quot;'&quot; , &quot;).val(getdata[&quot; , &quot;'&quot; , &quot;days&quot; , &quot;'&quot; , &quot;]);
  }
});
$(&quot; , &quot;'&quot; , &quot;#umur_tahun&quot; , &quot;'&quot; , &quot;).keyup(function(){
  generateUmurToBirthday();  
});
$(&quot; , &quot;'&quot; , &quot;#umur_bulan&quot; , &quot;'&quot; , &quot;).keyup(function(){
  generateUmurToBirthday();
});
$(&quot; , &quot;'&quot; , &quot;#umur_hari&quot; , &quot;'&quot; , &quot;).keyup(function(){
  generateUmurToBirthday();
});

function generateUmurToBirthday()
{
  var bDay = moment().subtract($(&quot; , &quot;'&quot; , &quot;#umur_tahun&quot; , &quot;'&quot; , &quot;).val(), &quot; , &quot;'&quot; , &quot;years&quot; , &quot;'&quot; , &quot;);
  bDay.subtract($(&quot; , &quot;'&quot; , &quot;#umur_bulan&quot; , &quot;'&quot; , &quot;).val(), &quot; , &quot;'&quot; , &quot;months&quot; , &quot;'&quot; , &quot;);
  bDay.subtract($(&quot; , &quot;'&quot; , &quot;#umur_hari&quot; , &quot;'&quot; , &quot;).val(), &quot; , &quot;'&quot; , &quot;days&quot; , &quot;'&quot; , &quot;);
  bDay = bDay.format(&quot;DD-MM-YYYY&quot;);
  $(&quot; , &quot;'&quot; , &quot;#tanggal_lahir&quot; , &quot;'&quot; , &quot;).val(bDay);
}
$(&quot;input[name=&quot; , &quot;'&quot; , &quot;cari_pasien&quot; , &quot;'&quot; , &quot;&quot;).autocomplete({
    delay: 1500,
    source: function( request, response ) {
        $.ajax({
            url: &quot;https://demo.epuskesmas.id/pasien/create&quot;,
            method: &quot;GET&quot;,
            dataType: &quot;json&quot;,
            data: {
                autocomplete: &quot; , &quot;'&quot; , &quot;pasien&quot; , &quot;'&quot; , &quot;,
                search: {&quot; , &quot;'&quot; , &quot;cari_pasien&quot; , &quot;'&quot; , &quot;:request.term}
            },
            success: function(data) {
                response(data);
            },
            error: function (xhr, ajaxOptions, thrownError) {
                console.log(xhr.status);
                console.log(thrownError);
            }
        });
    },
    focus: function(event,ui){
        $(this).parent().removeClass(&quot; , &quot;'&quot; , &quot;has-success&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;has-error&quot; , &quot;'&quot; , &quot;);
        return false;
    },
    select: function(event,ui){
        $(this).parent().removeClass(&quot; , &quot;'&quot; , &quot;has-error&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;has-success&quot; , &quot;'&quot; , &quot;);
        getPasienData(ui.item);     
    },
    minLength: 5
});


$(&quot; , &quot;'&quot; , &quot;.date&quot; , &quot;'&quot; , &quot;).datetimepicker({
      format:&quot; , &quot;'&quot; , &quot;DD-MM-YYYY&quot; , &quot;'&quot; , &quot;,
      useCurrent: false,
  });

function getPasienData(item)
{
    $.ajax({
        url: &quot;https://demo.epuskesmas.id/pasien/create&quot;,
        method: &quot;GET&quot;,
        dataType: &quot;json&quot;,
        data: {
            autocomplete: &quot; , &quot;'&quot; , &quot;get_pasien&quot; , &quot;'&quot; , &quot;,
            search: {&quot; , &quot;'&quot; , &quot;get_pasien&quot; , &quot;'&quot; , &quot;:item.id}
        },
        success: function(data) {
            if(data.length > 0) {
                data.forEach(function(items,index){
                    setDataDariKota(items);
                });
                umur = generateUmur()
                $(&quot; , &quot;'&quot; , &quot;#umur_tahun&quot; , &quot;'&quot; , &quot;).val(umur[&quot; , &quot;'&quot; , &quot;years&quot; , &quot;'&quot; , &quot;]);
                $(&quot; , &quot;'&quot; , &quot;#umur_bulan&quot; , &quot;'&quot; , &quot;).val(umur[&quot; , &quot;'&quot; , &quot;months&quot; , &quot;'&quot; , &quot;]);
                $(&quot; , &quot;'&quot; , &quot;#umur_hari&quot; , &quot;'&quot; , &quot;).val(umur[&quot; , &quot;'&quot; , &quot;days&quot; , &quot;'&quot; , &quot;]);
            }
        },
        error: function (xhr, ajaxOptions, thrownError) {
            console.log(xhr.status);
            console.log(thrownError);
        }
    });
}

function generateUmur()
{
  var arr = [];
  var tes = $(&quot; , &quot;'&quot; , &quot;#tanggal_lahir&quot; , &quot;'&quot; , &quot;).val();
  var real = moment(&quot;&quot;+tes+&quot;&quot;, &quot;DD-MM-YYYY&quot;);
  var now = moment();
  var years = now.diff(real, &quot; , &quot;'&quot; , &quot;years&quot; , &quot;'&quot; , &quot;);
  real.add(years, &quot; , &quot;'&quot; , &quot;years&quot; , &quot;'&quot; , &quot;);

  var months = now.diff(real, &quot; , &quot;'&quot; , &quot;months&quot; , &quot;'&quot; , &quot;);
  real.add(months, &quot; , &quot;'&quot; , &quot;months&quot; , &quot;'&quot; , &quot;);

  var days = now.diff(real, &quot; , &quot;'&quot; , &quot;days&quot; , &quot;'&quot; , &quot;);

  arr[&quot; , &quot;'&quot; , &quot;years&quot; , &quot;'&quot; , &quot;] = years;
  arr[&quot; , &quot;'&quot; , &quot;months&quot; , &quot;'&quot; , &quot;] = months;
  arr[&quot; , &quot;'&quot; , &quot;days&quot; , &quot;'&quot; , &quot;] = days;
  
  return arr;
}

function generateUmurByDate(data)
{
  var arr = [];
  var birthdate = moment(&quot;&quot;+data+&quot;&quot;, &quot;DD-MM-YYYY&quot;);
  var now = moment();

  var years = now.diff(birthdate, &quot; , &quot;'&quot; , &quot;year&quot; , &quot;'&quot; , &quot;);
  birthdate.add(years, &quot; , &quot;'&quot; , &quot;years&quot; , &quot;'&quot; , &quot;);

  var months = now.diff(birthdate, &quot; , &quot;'&quot; , &quot;months&quot; , &quot;'&quot; , &quot;);
  birthdate.add(months, &quot; , &quot;'&quot; , &quot;months&quot; , &quot;'&quot; , &quot;);

  var days = now.diff(birthdate, &quot; , &quot;'&quot; , &quot;days&quot; , &quot;'&quot; , &quot;);
  arr[&quot; , &quot;'&quot; , &quot;years&quot; , &quot;'&quot; , &quot;] = years;
  arr[&quot; , &quot;'&quot; , &quot;months&quot; , &quot;'&quot; , &quot;] = months;
  arr[&quot; , &quot;'&quot; , &quot;days&quot; , &quot;'&quot; , &quot;] = days;
  
  return arr;
}
$(&quot;input[name=&quot; , &quot;'&quot; , &quot;MPasien[no_kk]&quot; , &quot;'&quot; , &quot;&quot;).autocomplete({
    delay: 1000,
    source: function( request, response ) {
        $.ajax({
            url: &quot;https://demo.epuskesmas.id/pasien/create&quot;,
            method: &quot;GET&quot;,
            dataType: &quot;json&quot;,
            data: {
                autocomplete: &quot; , &quot;'&quot; , &quot;no_kk&quot; , &quot;'&quot; , &quot;,
                search: {&quot; , &quot;'&quot; , &quot;no_kk&quot; , &quot;'&quot; , &quot;:request.term}
            },
            success: function(data) {
                response(data);
            },
            error: function (xhr, ajaxOptions, thrownError) {
                console.log(xhr.status);
                console.log(thrownError);
            }
        });
    },
    focus: function(event,ui){
        $(this).parent().removeClass(&quot; , &quot;'&quot; , &quot;has-success&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;has-error&quot; , &quot;'&quot; , &quot;);
        return false;
    },
    select: function(event,ui){
        $(this).parent().removeClass(&quot; , &quot;'&quot; , &quot;has-error&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;has-success&quot; , &quot;'&quot; , &quot;);
        setDataDariKK(ui.item);
    },
    minLength: 5
});

function setDataDariKota(item)
{
    setDataDariKK(item);
    $(&quot; , &quot;'&quot; , &quot;#created_id&quot; , &quot;'&quot; , &quot;).html(item.id);

    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[verified_by]&quot;]&quot; , &quot;'&quot; , &quot;).prop(&quot;checked&quot;, false);
    if(item.verified_by != &quot;&quot; &amp;&amp; item.verified_by != null){
        $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[nik]&quot;]&quot; , &quot;'&quot; , &quot;).attr(&quot;required&quot;, false);
        $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[verified_by]&quot;]&quot; , &quot;'&quot; , &quot;).prop(&quot;checked&quot;, true);
    }else{
        $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[nik]&quot;]&quot; , &quot;'&quot; , &quot;).attr(&quot;required&quot;, true);
        $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[verified_by]&quot;]&quot; , &quot;'&quot; , &quot;).prop(&quot;checked&quot;, false);
    }
    setRequiredForm(false);

    if(item.ktp_alamat_berbeda == 1 || item.ktp_alamat_berbeda == true){
        $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[ktp_propinsi_id]&quot;]&quot; , &quot;'&quot; , &quot;).attr(&quot;required&quot;, false);
        if($(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[ktp_alamat_berbeda]&quot;]&quot; , &quot;'&quot; , &quot;).is(&quot; , &quot;'&quot; , &quot;:not(:checked)&quot; , &quot;'&quot; , &quot;)){
            $(&quot;#collapseAlamatKtp&quot;).collapse(&quot;show&quot;);
            $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[ktp_alamat_berbeda]&quot;]&quot; , &quot;'&quot; , &quot;).prop(&quot;checked&quot;, true);
        }
    }else{
        $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[ktp_propinsi_id]&quot;]&quot; , &quot;'&quot; , &quot;).attr(&quot;required&quot;, true);
        $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[ktp_alamat_berbeda]&quot;]&quot; , &quot;'&quot; , &quot;).prop(&quot;checked&quot;, false);
        $(&quot; , &quot;'&quot; , &quot;#collapseAlamatKtp&quot; , &quot;'&quot; , &quot;).collapse(&quot;hide&quot;);
    }
    setRequiredFormKtp(false);

    $(&quot; , &quot;'&quot; , &quot;select[name=&quot;MPasien[asuransi_id]&quot;]&quot; , &quot;'&quot; , &quot;).val(item.asuransi_id).change();
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[no_asuransi]&quot;]&quot; , &quot;'&quot; , &quot;).val(item.no_asuransi);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[id]&quot;]&quot; , &quot;'&quot; , &quot;).val(item.id);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[nama]&quot;]&quot; , &quot;'&quot; , &quot;).val(item.nama);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[nik]&quot;]&quot; , &quot;'&quot; , &quot;).val(item.nik);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[no_rm_lama]&quot;]&quot; , &quot;'&quot; , &quot;).val(item.no_rm_lama);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[jenis_kelamin]&quot;][value=&quot;&quot; , &quot;'&quot; , &quot;+item.jenis_kelamin+&quot; , &quot;'&quot; , &quot;&quot;]&quot; , &quot;'&quot; , &quot;).prop(&quot;checked&quot;, true);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[tempat_lahir]&quot;]&quot; , &quot;'&quot; , &quot;).val(item.tempat_lahir);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[tanggal_lahir]&quot;]&quot; , &quot;'&quot; , &quot;).val(item.tanggal_lahir);
    $(&quot; , &quot;'&quot; , &quot;select[name=&quot;MPasien[gol_darah]&quot;]&quot; , &quot;'&quot; , &quot;).val(item.gol_darah);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[no_hp]&quot;]&quot; , &quot;'&quot; , &quot;).val(item.no_hp);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[pekerjaan_id]&quot;]&quot; , &quot;'&quot; , &quot;).val(item.pekerjaan_id);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[instansi]&quot;]&quot; , &quot;'&quot; , &quot;).val(item.instansi);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;pekerjaan_nama&quot;]&quot; , &quot;'&quot; , &quot;).val(item.pekerjaan_nama);
    $(&quot; , &quot;'&quot; , &quot;select[name=&quot;MPasien[agama]&quot;]&quot; , &quot;'&quot; , &quot;).val(item.agama);
    $(&quot; , &quot;'&quot; , &quot;select[name=&quot;MPasien[pendidikan]&quot;]&quot; , &quot;'&quot; , &quot;).val(item.pendidikan);
    $(&quot; , &quot;'&quot; , &quot;select[name=&quot;MPasien[status_perkawinan]&quot;]&quot; , &quot;'&quot; , &quot;).val(item.status_perkawinan);
    $(&quot; , &quot;'&quot; , &quot;select[name=&quot;MPasien[status_keluarga]&quot;]&quot; , &quot;'&quot; , &quot;).val(item.status_keluarga);
    $(&quot; , &quot;'&quot; , &quot;select[name=&quot;MPasien[warganegara]&quot;]&quot; , &quot;'&quot; , &quot;).val(item.warganegara);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[nama_ibu]&quot;]&quot; , &quot;'&quot; , &quot;).val(item.nama_ibu);

    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[ktp_propinsi_id]&quot;]&quot; , &quot;'&quot; , &quot;).val(item.ktp_propinsi_id);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;ktp_propinsi_nama&quot;]&quot; , &quot;'&quot; , &quot;).val(item.ktp_propinsi_nama);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[ktp_kota_id]&quot;]&quot; , &quot;'&quot; , &quot;).val(item.ktp_kota_id);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;ktp_kota_nama&quot;]&quot; , &quot;'&quot; , &quot;).val(item.ktp_kota_nama);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[ktp_kecamatan_id]&quot;]&quot; , &quot;'&quot; , &quot;).val(item.ktp_kecamatan_id);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;ktp_kecamatan_nama&quot;]&quot; , &quot;'&quot; , &quot;).val(item.ktp_kecamatan_nama);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[ktp_kelurahan_id]&quot;]&quot; , &quot;'&quot; , &quot;).val(item.ktp_kelurahan_id);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;ktp_kelurahan_nama&quot;]&quot; , &quot;'&quot; , &quot;).val(item.ktp_kelurahan_nama);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[ktp_dusun_id]&quot;]&quot; , &quot;'&quot; , &quot;).val(item.ktp_dusun_id);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;ktp_dusun_nama&quot;]&quot; , &quot;'&quot; , &quot;).val(item.ktp_dusun_nama);
    $(&quot; , &quot;'&quot; , &quot;textarea[name=&quot;MPasien[ktp_alamat]&quot;]&quot; , &quot;'&quot; , &quot;).val(item.ktp_alamat);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[ktp_rt]&quot;]&quot; , &quot;'&quot; , &quot;).val(item.ktp_rt);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[ktp_rw]&quot;]&quot; , &quot;'&quot; , &quot;).val(item.ktp_rw);

    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[imd]&quot;]&quot; , &quot;'&quot; , &quot;).prop(&quot;checked&quot;, false);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[bb_lahir]&quot;]&quot; , &quot;'&quot; , &quot;).val(item.bb_lahir);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[tb_lahir]&quot;]&quot; , &quot;'&quot; , &quot;).val(item.tb_lahir);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[imd]&quot;][value=&quot;&quot; , &quot;'&quot; , &quot;+ item.imd +&quot; , &quot;'&quot; , &quot;&quot;]&quot; , &quot;'&quot; , &quot;).prop(&quot; , &quot;'&quot; , &quot;checked&quot; , &quot;'&quot; , &quot;, true);

}

function setDataDariKK(item)
{
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[propinsi_id]&quot;]&quot; , &quot;'&quot; , &quot;).val(item.propinsi_id);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;propinsi_nama&quot;]&quot; , &quot;'&quot; , &quot;).val(item.propinsi_nama);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[kota_id]&quot;]&quot; , &quot;'&quot; , &quot;).val(item.kota_id);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;kota_nama&quot;]&quot; , &quot;'&quot; , &quot;).val(item.kota_nama);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[kecamatan_id]&quot;]&quot; , &quot;'&quot; , &quot;).val(item.kecamatan_id);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;kecamatan_nama&quot;]&quot; , &quot;'&quot; , &quot;).val(item.kecamatan_nama);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[kelurahan_id]&quot;]&quot; , &quot;'&quot; , &quot;).val(item.kelurahan_id);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;kelurahan_nama&quot;]&quot; , &quot;'&quot; , &quot;).val(item.kelurahan_nama);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[dusun_id]&quot;]&quot; , &quot;'&quot; , &quot;).val(item.dusun_id);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;dusun_nama&quot;]&quot; , &quot;'&quot; , &quot;).val(item.dusun_nama);
    $(&quot; , &quot;'&quot; , &quot;textarea[name=&quot;MPasien[alamat]&quot;]&quot; , &quot;'&quot; , &quot;).val(item.alamat);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[rt]&quot;]&quot; , &quot;'&quot; , &quot;).val(item.rt);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[rw]&quot;]&quot; , &quot;'&quot; , &quot;).val(item.rw);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[nama_ayah]&quot;]&quot; , &quot;'&quot; , &quot;).val(item.nama_ayah);
}

$(&quot;input[name=&quot; , &quot;'&quot; , &quot;pekerjaan_nama&quot; , &quot;'&quot; , &quot;&quot;).autocomplete({
    delay: 1000,
    source: function( request, response ) {
        $.ajax({
            url: &quot;https://demo.epuskesmas.id/pasien/create&quot;,
            method: &quot;GET&quot;,
            dataType: &quot;json&quot;,
            data: {
                autocomplete: &quot; , &quot;'&quot; , &quot;pekerjaan&quot; , &quot;'&quot; , &quot;,
                search: {&quot; , &quot;'&quot; , &quot;pekerjaan_nama&quot; , &quot;'&quot; , &quot;:request.term}
            },
            success: function(data) {
                response(data);
            },
            error: function (xhr, ajaxOptions, thrownError) {
                console.log(xhr.status);
                console.log(thrownError);
            }
        });
    },
    focus: function(event,ui){
        $(this).parent().removeClass(&quot; , &quot;'&quot; , &quot;has-success&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;has-error&quot; , &quot;'&quot; , &quot;);
        return false;
    },
    select: function(event,ui){
        $(this).parent().removeClass(&quot; , &quot;'&quot; , &quot;has-error&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;has-success&quot; , &quot;'&quot; , &quot;);
        $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[pekerjaan_id]&quot;]&quot; , &quot;'&quot; , &quot;).val(ui.item.id);
    },
    minLength: 3
});
$(&quot;input[name=&quot; , &quot;'&quot; , &quot;pekerjaan_suami_nama&quot; , &quot;'&quot; , &quot;&quot;).autocomplete({
    delay: 1000,
    source: function( request, response ) {
        $.ajax({
            url: &quot;https://demo.epuskesmas.id/pasien/create&quot;,
            method: &quot;GET&quot;,
            dataType: &quot;json&quot;,
            data: {
                autocomplete: &quot; , &quot;'&quot; , &quot;pekerjaan&quot; , &quot;'&quot; , &quot;,
                search: {&quot; , &quot;'&quot; , &quot;pekerjaan_nama&quot; , &quot;'&quot; , &quot;:request.term}
            },
            success: function(data) {
                response(data);
            },
            error: function (xhr, ajaxOptions, thrownError) {
                console.log(xhr.status);
                console.log(thrownError);
            }
        });
    },
    focus: function(event,ui){
        $(this).parent().removeClass(&quot; , &quot;'&quot; , &quot;has-success&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;has-error&quot; , &quot;'&quot; , &quot;);
        return false;
    },
    select: function(event,ui){
        $(this).parent().removeClass(&quot; , &quot;'&quot; , &quot;has-error&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;has-success&quot; , &quot;'&quot; , &quot;);
        $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[pekerjaan_suami_id]&quot;]&quot; , &quot;'&quot; , &quot;).val(ui.item.id);
    },
    minLength: 3
});
$(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[instansi]&quot;]&quot; , &quot;'&quot; , &quot;).autocomplete({
    delay: 1000,
    source: function( request, response ) {
        $.ajax({
            url: &quot;https://demo.epuskesmas.id/pasien/create&quot;,
            method: &quot;GET&quot;,
            dataType: &quot;json&quot;,
            data: {
                autocomplete: &quot; , &quot;'&quot; , &quot;instansi&quot; , &quot;'&quot; , &quot;,
                search: {&quot; , &quot;'&quot; , &quot;instansi&quot; , &quot;'&quot; , &quot;:request.term}
            },
            success: function(data) {
                response(data);
            },
            error: function (xhr, ajaxOptions, thrownError) {
                console.log(xhr.status);
                console.log(thrownError);
            }
        });
    },
    focus: function(event,ui){
        return false;
    },
    select: function(event,ui){
        $(this).parent().removeClass(&quot; , &quot;'&quot; , &quot;has-error&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;has-success&quot; , &quot;'&quot; , &quot;);
    },
    minLength: 3
});

function setWilayah(item)
{
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[propinsi_id]&quot;]&quot; , &quot;'&quot; , &quot;).val(item.propinsi_id);
    $(&quot; , &quot;'&quot; , &quot;input[name=propinsi_nama]&quot; , &quot;'&quot; , &quot;).val(item.propinsi_nama);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[kota_id]&quot;]&quot; , &quot;'&quot; , &quot;).val(item.kota_id);
    $(&quot; , &quot;'&quot; , &quot;input[name=kota_nama]&quot; , &quot;'&quot; , &quot;).val(item.kota_nama);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[kecamatan_id]&quot;]&quot; , &quot;'&quot; , &quot;).val(item.kecamatan_id);
    $(&quot; , &quot;'&quot; , &quot;input[name=kecamatan_nama]&quot; , &quot;'&quot; , &quot;).val(item.kecamatan_nama);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[kelurahan_id]&quot;]&quot; , &quot;'&quot; , &quot;).val(item.kelurahan_id);
    $(&quot; , &quot;'&quot; , &quot;input[name=kelurahan_nama]&quot; , &quot;'&quot; , &quot;).val(item.kelurahan_nama);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[dusun_id]&quot;]&quot; , &quot;'&quot; , &quot;).val(item.dusun_id);
    $(&quot; , &quot;'&quot; , &quot;input[name=dusun_nama]&quot; , &quot;'&quot; , &quot;).val(item.dusun_nama);
}

function setWilayahKtp(item)
{
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[ktp_propinsi_id]&quot;]&quot; , &quot;'&quot; , &quot;).val(item.propinsi_id);
    $(&quot; , &quot;'&quot; , &quot;input[name=ktp_propinsi_nama]&quot; , &quot;'&quot; , &quot;).val(item.propinsi_nama);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[ktp_kota_id]&quot;]&quot; , &quot;'&quot; , &quot;).val(item.kota_id);
    $(&quot; , &quot;'&quot; , &quot;input[name=ktp_kota_nama]&quot; , &quot;'&quot; , &quot;).val(item.kota_nama);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[ktp_kecamatan_id]&quot;]&quot; , &quot;'&quot; , &quot;).val(item.kecamatan_id);
    $(&quot; , &quot;'&quot; , &quot;input[name=ktp_kecamatan_nama]&quot; , &quot;'&quot; , &quot;).val(item.kecamatan_nama);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[ktp_kelurahan_id]&quot;]&quot; , &quot;'&quot; , &quot;).val(item.kelurahan_id);
    $(&quot; , &quot;'&quot; , &quot;input[name=ktp_kelurahan_nama]&quot; , &quot;'&quot; , &quot;).val(item.kelurahan_nama);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[ktp_dusun_id]&quot;]&quot; , &quot;'&quot; , &quot;).val(item.dusun_id);
    $(&quot; , &quot;'&quot; , &quot;input[name=ktp_dusun_nama]&quot; , &quot;'&quot; , &quot;).val(item.dusun_nama);
}

$(&quot;input[name=&quot; , &quot;'&quot; , &quot;propinsi_nama&quot; , &quot;'&quot; , &quot;&quot;).autocomplete({
    delay: 1000,
    source: function( request, response ) {
        $.ajax({
            url: &quot;https://demo.epuskesmas.id/pasien/create&quot;,
            method: &quot;GET&quot;,
            dataType: &quot;json&quot;,
            data: {
                autocomplete: &quot; , &quot;'&quot; , &quot;propinsi&quot; , &quot;'&quot; , &quot;,
                search: {&quot; , &quot;'&quot; , &quot;propinsi_nama&quot; , &quot;'&quot; , &quot;:request.term}
            },
            success: function(data) {
                response(data);
            },
            error: function (xhr, ajaxOptions, thrownError) {
                console.log(xhr.status);
                console.log(thrownError);
            }
        });
    },
    focus: function(event,ui){
        $(this).parent().removeClass(&quot; , &quot;'&quot; , &quot;has-success&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;has-error&quot; , &quot;'&quot; , &quot;);
        return false;
    },
    select: function(event,ui){
        setWilayah(ui.item);
        $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[propinsi_id]&quot;]&quot; , &quot;'&quot; , &quot;).val(ui.item.id);
        $(this).parent().removeClass(&quot; , &quot;'&quot; , &quot;has-error&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;has-success&quot; , &quot;'&quot; , &quot;);
    },
    minLength: 3
});

$(&quot;input[name=&quot; , &quot;'&quot; , &quot;kota_nama&quot; , &quot;'&quot; , &quot;]&quot;).autocomplete({
    delay: 1000,
    source: function( request, response ) {
        $.ajax({
            url: &quot;https://demo.epuskesmas.id/pasien/create&quot;,
            method: &quot;GET&quot;,
            dataType: &quot;json&quot;,
            data: {
                autocomplete: &quot; , &quot;'&quot; , &quot;kota&quot; , &quot;'&quot; , &quot;,
                search: {
                    &quot; , &quot;'&quot; , &quot;propinsi_id&quot; , &quot;'&quot; , &quot;:$(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[propinsi_id]&quot;]&quot; , &quot;'&quot; , &quot;).val(),
                    &quot; , &quot;'&quot; , &quot;kota_nama&quot; , &quot;'&quot; , &quot;:request.term
                }
            },
            success: function(data) {
                response(data);
            },
            error: function (xhr, ajaxOptions, thrownError) {
                console.log(xhr.status);
                console.log(thrownError);
            }
        });
    },
    focus: function(event,ui){
        $(this).parent().removeClass(&quot; , &quot;'&quot; , &quot;has-success&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;has-error&quot; , &quot;'&quot; , &quot;);
        return false;
    },
    select: function(event,ui){
        setWilayah(ui.item);
        $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[kota_id]&quot;]&quot; , &quot;'&quot; , &quot;).val(ui.item.id);
        $(this).parent().removeClass(&quot; , &quot;'&quot; , &quot;has-error&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;has-success&quot; , &quot;'&quot; , &quot;);
    },
    minLength: 3
});

$(&quot;input[name=&quot; , &quot;'&quot; , &quot;kecamatan_nama&quot; , &quot;'&quot; , &quot;]&quot;).autocomplete({
    delay: 1000,
    source: function( request, response ) {
        $.ajax({
            url: &quot;https://demo.epuskesmas.id/pasien/create&quot;,
            method: &quot;GET&quot;,
            dataType: &quot;json&quot;,
            data: {
                autocomplete: &quot; , &quot;'&quot; , &quot;kecamatan&quot; , &quot;'&quot; , &quot;,
                search: {
                    &quot; , &quot;'&quot; , &quot;kota_id&quot; , &quot;'&quot; , &quot;:$(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[kota_id]&quot;]&quot; , &quot;'&quot; , &quot;).val(),
                    &quot; , &quot;'&quot; , &quot;kecamatan_nama&quot; , &quot;'&quot; , &quot;:request.term
                }
            },
            success: function(data) {
                response(data);
            },
            error: function (xhr, ajaxOptions, thrownError) {
                console.log(xhr.status);
                console.log(thrownError);
            }
        });
    },
    focus: function(event,ui){
        $(this).parent().removeClass(&quot; , &quot;'&quot; , &quot;has-success&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;has-error&quot; , &quot;'&quot; , &quot;);
        return false;
    },
    select: function(event,ui){
        setWilayah(ui.item);
        $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[kecamatan_id]&quot;]&quot; , &quot;'&quot; , &quot;).val(ui.item.id);
        $(this).parent().removeClass(&quot; , &quot;'&quot; , &quot;has-error&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;has-success&quot; , &quot;'&quot; , &quot;);
    },
    minLength: 3
});

$(&quot;input[name=&quot; , &quot;'&quot; , &quot;kelurahan_nama&quot; , &quot;'&quot; , &quot;]&quot;).autocomplete({
    delay: 1000,
    source: function( request, response ) {
        $.ajax({
            url: &quot;https://demo.epuskesmas.id/pasien/create&quot;,
            method: &quot;GET&quot;,
            dataType: &quot;json&quot;,
            data: {
                autocomplete: &quot; , &quot;'&quot; , &quot;kelurahan&quot; , &quot;'&quot; , &quot;,
                search: {
                    &quot; , &quot;'&quot; , &quot;kecamatan_id&quot; , &quot;'&quot; , &quot;:$(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[kecamatan_id]&quot;]&quot; , &quot;'&quot; , &quot;).val(),
                    &quot; , &quot;'&quot; , &quot;kelurahan_nama&quot; , &quot;'&quot; , &quot;:request.term
                }
            },
            success: function(data) {
                response(data);
            },
            error: function (xhr, ajaxOptions, thrownError) {
                console.log(xhr.status);
                console.log(thrownError);
            }
        });
    },
    focus: function(event,ui){
        $(this).parent().removeClass(&quot; , &quot;'&quot; , &quot;has-success&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;has-error&quot; , &quot;'&quot; , &quot;);
        return false;
    },
    select: function(event,ui){
        setWilayah(ui.item);
        $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[kelurahan_id]&quot;]&quot; , &quot;'&quot; , &quot;).val(ui.item.id);
        $(this).parent().removeClass(&quot; , &quot;'&quot; , &quot;has-error&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;has-success&quot; , &quot;'&quot; , &quot;);
    },
    minLength: 3
});

$(&quot;input[name=&quot; , &quot;'&quot; , &quot;dusun_nama&quot; , &quot;'&quot; , &quot;]&quot;).autocomplete({
    delay: 1000,
    source: function( request, response ) {
        $.ajax({
            url: &quot;https://demo.epuskesmas.id/pasien/create&quot;,
            method: &quot;GET&quot;,
            dataType: &quot;json&quot;,
            data: {
                autocomplete: &quot; , &quot;'&quot; , &quot;dusun&quot; , &quot;'&quot; , &quot;,
                search: {
                    &quot; , &quot;'&quot; , &quot;kelurahan_id&quot; , &quot;'&quot; , &quot;:$(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[kelurahan_id]&quot;]&quot; , &quot;'&quot; , &quot;).val(),
                    &quot; , &quot;'&quot; , &quot;dusun_nama&quot; , &quot;'&quot; , &quot;:request.term
                }
            },
            success: function(data) {
                response(data);
            },
            error: function (xhr, ajaxOptions, thrownError) {
                console.log(xhr.status);
                console.log(thrownError);
            }
        });
    },
    focus: function(event,ui){
        $(this).parent().removeClass(&quot; , &quot;'&quot; , &quot;has-success&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;has-error&quot; , &quot;'&quot; , &quot;);
        return false;
    },
    select: function(event,ui){
        setWilayah(ui.item);
        $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[dusun_id]&quot;]&quot; , &quot;'&quot; , &quot;).val(ui.item.id);
        $(this).parent().removeClass(&quot; , &quot;'&quot; , &quot;has-error&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;has-success&quot; , &quot;'&quot; , &quot;);
    },
    minLength: 3
});

$(&quot;input[name=&quot; , &quot;'&quot; , &quot;ktp_propinsi_nama&quot; , &quot;'&quot; , &quot;&quot;).autocomplete({
    delay: 1000,
    source: function( request, response ) {
        $.ajax({
            url: &quot;https://demo.epuskesmas.id/pasien/create&quot;,
            method: &quot;GET&quot;,
            dataType: &quot;json&quot;,
            data: {
                autocomplete: &quot; , &quot;'&quot; , &quot;propinsi&quot; , &quot;'&quot; , &quot;,
                search: {&quot; , &quot;'&quot; , &quot;propinsi_nama&quot; , &quot;'&quot; , &quot;:request.term}
            },
            success: function(data) {
                response(data);
            },
            error: function (xhr, ajaxOptions, thrownError) {
                console.log(xhr.status);
                console.log(thrownError);
            }
        });
    },
    focus: function(event,ui){
        $(this).parent().removeClass(&quot; , &quot;'&quot; , &quot;has-success&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;has-error&quot; , &quot;'&quot; , &quot;);
        return false;
    },
    select: function(event,ui){
        setWilayahKtp(ui.item);
        $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[ktp_propinsi_id]&quot;]&quot; , &quot;'&quot; , &quot;).val(ui.item.id);
        $(this).parent().removeClass(&quot; , &quot;'&quot; , &quot;has-error&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;has-success&quot; , &quot;'&quot; , &quot;);
    },
    minLength: 3
});

$(&quot;input[name=&quot; , &quot;'&quot; , &quot;ktp_kota_nama&quot; , &quot;'&quot; , &quot;]&quot;).autocomplete({
    delay: 1000,
    source: function( request, response ) {
        $.ajax({
            url: &quot;https://demo.epuskesmas.id/pasien/create&quot;,
            method: &quot;GET&quot;,
            dataType: &quot;json&quot;,
            data: {
                autocomplete: &quot; , &quot;'&quot; , &quot;kota&quot; , &quot;'&quot; , &quot;,
                search: {
                    &quot; , &quot;'&quot; , &quot;propinsi_id&quot; , &quot;'&quot; , &quot;:$(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[ktp_propinsi_id]&quot;]&quot; , &quot;'&quot; , &quot;).val(),
                    &quot; , &quot;'&quot; , &quot;kota_nama&quot; , &quot;'&quot; , &quot;:request.term
                }
            },
            success: function(data) {
                response(data);
            },
            error: function (xhr, ajaxOptions, thrownError) {
                console.log(xhr.status);
                console.log(thrownError);
            }
        });
    },
    focus: function(event,ui){
        $(this).parent().removeClass(&quot; , &quot;'&quot; , &quot;has-success&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;has-error&quot; , &quot;'&quot; , &quot;);
        return false;
    },
    select: function(event,ui){
        setWilayahKtp(ui.item);
        $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[ktp_kota_id]&quot;]&quot; , &quot;'&quot; , &quot;).val(ui.item.id);
        $(this).parent().removeClass(&quot; , &quot;'&quot; , &quot;has-error&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;has-success&quot; , &quot;'&quot; , &quot;);
    },
    minLength: 3
});

$(&quot;input[name=&quot; , &quot;'&quot; , &quot;ktp_kecamatan_nama&quot; , &quot;'&quot; , &quot;]&quot;).autocomplete({
    delay: 1000,
    source: function( request, response ) {
        $.ajax({
            url: &quot;https://demo.epuskesmas.id/pasien/create&quot;,
            method: &quot;GET&quot;,
            dataType: &quot;json&quot;,
            data: {
                autocomplete: &quot; , &quot;'&quot; , &quot;kecamatan&quot; , &quot;'&quot; , &quot;,
                search: {
                    &quot; , &quot;'&quot; , &quot;kota_id&quot; , &quot;'&quot; , &quot;:$(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[ktp_kota_id]&quot;]&quot; , &quot;'&quot; , &quot;).val(),
                    &quot; , &quot;'&quot; , &quot;kecamatan_nama&quot; , &quot;'&quot; , &quot;:request.term
                }
            },
            success: function(data) {
                response(data);
            },
            error: function (xhr, ajaxOptions, thrownError) {
                console.log(xhr.status);
                console.log(thrownError);
            }
        });
    },
    focus: function(event,ui){
        $(this).parent().removeClass(&quot; , &quot;'&quot; , &quot;has-success&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;has-error&quot; , &quot;'&quot; , &quot;);
        return false;
    },
    select: function(event,ui){
        setWilayahKtp(ui.item);
        $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[ktp_kecamatan_id]&quot;]&quot; , &quot;'&quot; , &quot;).val(ui.item.id);
        $(this).parent().removeClass(&quot; , &quot;'&quot; , &quot;has-error&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;has-success&quot; , &quot;'&quot; , &quot;);
    },
    minLength: 3
});

$(&quot;input[name=&quot; , &quot;'&quot; , &quot;ktp_kelurahan_nama&quot; , &quot;'&quot; , &quot;]&quot;).autocomplete({
    delay: 1000,
    source: function( request, response ) {
        $.ajax({
            url: &quot;https://demo.epuskesmas.id/pasien/create&quot;,
            method: &quot;GET&quot;,
            dataType: &quot;json&quot;,
            data: {
                autocomplete: &quot; , &quot;'&quot; , &quot;kelurahan&quot; , &quot;'&quot; , &quot;,
                search: {
                    &quot; , &quot;'&quot; , &quot;kecamatan_id&quot; , &quot;'&quot; , &quot;:$(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[ktp_kecamatan_id]&quot;]&quot; , &quot;'&quot; , &quot;).val(),
                    &quot; , &quot;'&quot; , &quot;kelurahan_nama&quot; , &quot;'&quot; , &quot;:request.term
                }
            },
            success: function(data) {
                response(data);
            },
            error: function (xhr, ajaxOptions, thrownError) {
                console.log(xhr.status);
                console.log(thrownError);
            }
        });
    },
    focus: function(event,ui){
        $(this).parent().removeClass(&quot; , &quot;'&quot; , &quot;has-success&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;has-error&quot; , &quot;'&quot; , &quot;);
        return false;
    },
    select: function(event,ui){
        setWilayahKtp(ui.item);
        $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[ktp_kelurahan_id]&quot;]&quot; , &quot;'&quot; , &quot;).val(ui.item.id);
        $(this).parent().removeClass(&quot; , &quot;'&quot; , &quot;has-error&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;has-success&quot; , &quot;'&quot; , &quot;);
    },
    minLength: 3
});

$(&quot;input[name=&quot; , &quot;'&quot; , &quot;ktp_dusun_nama&quot; , &quot;'&quot; , &quot;]&quot;).autocomplete({
    delay: 1000,
    source: function( request, response ) {
        $.ajax({
            url: &quot;https://demo.epuskesmas.id/pasien/create&quot;,
            method: &quot;GET&quot;,
            dataType: &quot;json&quot;,
            data: {
                autocomplete: &quot; , &quot;'&quot; , &quot;dusun&quot; , &quot;'&quot; , &quot;,
                search: {
                    &quot; , &quot;'&quot; , &quot;kelurahan_id&quot; , &quot;'&quot; , &quot;:$(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[ktp_kelurahan_id]&quot;]&quot; , &quot;'&quot; , &quot;).val(),
                    &quot; , &quot;'&quot; , &quot;dusun_nama&quot; , &quot;'&quot; , &quot;:request.term
                }
            },
            success: function(data) {
                response(data);
            },
            error: function (xhr, ajaxOptions, thrownError) {
                console.log(xhr.status);
                console.log(thrownError);
            }
        });
    },
    focus: function(event,ui){
        $(this).parent().removeClass(&quot; , &quot;'&quot; , &quot;has-success&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;has-error&quot; , &quot;'&quot; , &quot;);
        return false;
    },
    select: function(event,ui){
        setWilayahKtp(ui.item);
        $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[ktp_dusun_id]&quot;]&quot; , &quot;'&quot; , &quot;).val(ui.item.id);
        $(this).parent().removeClass(&quot; , &quot;'&quot; , &quot;has-error&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;has-success&quot; , &quot;'&quot; , &quot;);
    },
    minLength: 3
});

function setDataDariApiBpjs(peserta)
{
    var style = peserta.ketAktif == &quot;AKTIF&quot; ? &quot;text-success&quot; : &quot;text-danger&quot;;
    var status = &quot;text-danger&quot;;
    if(peserta.status == &quot; , &quot;'&quot; , &quot;Sesuai Faskes&quot; , &quot;'&quot; , &quot;){
        status = &quot;text-success&quot;;
    }else if(peserta.status == &quot; , &quot;'&quot; , &quot;Faskes Jejaring&quot; , &quot;'&quot; , &quot;){
        status = &quot;text-info&quot;;
    }
    html = &quot; , &quot;'&quot; , &quot;Status: &lt;b>&lt;span class=&quot; , &quot;'&quot; , &quot;+style+&quot; , &quot;'&quot; , &quot;>&quot; , &quot;'&quot; , &quot;+peserta.ketAktif+&quot; , &quot;'&quot; , &quot;&lt;\/span>&lt;\/b>&lt;br>&quot; , &quot;'&quot; , &quot;
            +&quot; , &quot;'&quot; , &quot;Kode Provider: &quot; , &quot;'&quot; , &quot;+peserta.kdProviderPst.kdProvider+&quot; , &quot;'&quot; , &quot;&lt;br>&quot; , &quot;'&quot; , &quot;
            +&quot; , &quot;'&quot; , &quot;Nama Provider: &quot; , &quot;'&quot; , &quot;+peserta.kdProviderPst.nmProvider+&quot; , &quot;'&quot; , &quot;&lt;br>&quot; , &quot;'&quot; , &quot;
            +&quot; , &quot;'&quot; , &quot;Nama Peserta: &quot; , &quot;'&quot; , &quot;+(peserta.nama).toUpperCase()+&quot; , &quot;'&quot; , &quot;&lt;br>&quot; , &quot;'&quot; , &quot;
            +&quot; , &quot;'&quot; , &quot;Kelas : &quot; , &quot;'&quot; , &quot;+peserta.jnsKelas.nama+&quot; , &quot;'&quot; , &quot;&lt;br>&quot; , &quot;'&quot; , &quot;
            +&quot; , &quot;'&quot; , &quot;Jenis Peserta : &quot; , &quot;'&quot; , &quot;+peserta.jnsPeserta.nama+&quot; , &quot;'&quot; , &quot;&lt;br>&quot; , &quot;'&quot; , &quot;
            +&quot; , &quot;'&quot; , &quot;Prolanis/PRB : &quot; , &quot;'&quot; , &quot;+(peserta.pstProl != &quot;&quot; ? &quot; , &quot;'&quot; , &quot;Ya(&quot; , &quot;'&quot; , &quot;+peserta.pstProl+&quot; , &quot;'&quot; , &quot;)&quot; , &quot;'&quot; , &quot; : &quot;Bukan&quot;)+(peserta.pstPrb != &quot;&quot; ? &quot; , &quot;'&quot; , &quot;/Ya(&quot; , &quot;'&quot; , &quot;+peserta.pstPrb+&quot; , &quot;'&quot; , &quot;)&quot; , &quot;'&quot; , &quot; : &quot;/Bukan&quot;)+&quot; , &quot;'&quot; , &quot;&lt;br>&quot; , &quot;'&quot; , &quot;
            +&quot; , &quot;'&quot; , &quot;Tunggakan: &quot; , &quot;'&quot; , &quot;+peserta.tunggakan+&quot; , &quot;'&quot; , &quot;&lt;br>&quot; , &quot;'&quot; , &quot;
            +&quot; , &quot;'&quot; , &quot;&lt;b>&lt;span class=&quot; , &quot;'&quot; , &quot;+status+&quot; , &quot;'&quot; , &quot;>&quot; , &quot;'&quot; , &quot;+peserta.status+&quot; , &quot;'&quot; , &quot;&lt;\/span>&lt;\/b>&lt;br>&quot; , &quot;'&quot; , &quot;;
    $(&quot; , &quot;'&quot; , &quot;#data_peserta_bpjs&quot; , &quot;'&quot; , &quot;).html(html);


    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[no_asuransi]&quot;]&quot; , &quot;'&quot; , &quot;).val(peserta.noKartu);
    if(peserta.noKTP != null){
        if(peserta.noKTP.trim() != &quot;&quot;){
            $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[nik]&quot;]&quot; , &quot;'&quot; , &quot;).val(peserta.noKTP);
        }
    }
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[nama]&quot;]&quot; , &quot;'&quot; , &quot;).val(peserta.nama);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[jenis_kelamin]&quot;][value=&quot;&quot; , &quot;'&quot; , &quot;+peserta.sex+&quot; , &quot;'&quot; , &quot;&quot;]&quot; , &quot;'&quot; , &quot;).attr(&quot;checked&quot;,true);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[tanggal_lahir]&quot;]&quot; , &quot;'&quot; , &quot;).val(peserta.tglLahir);
    $(&quot; , &quot;'&quot; , &quot;select[name=&quot;MPasien[gol_darah]&quot;]&quot; , &quot;'&quot; , &quot;).val(peserta.golDarah);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[no_hp]&quot;]&quot; , &quot;'&quot; , &quot;).val(peserta.noHP);
    $(&quot; , &quot;'&quot; , &quot;select[name=&quot;MPasien[status_keluarga]&quot;]&quot; , &quot;'&quot; , &quot;).val(peserta.hubunganKeluarga.toUpperCase());
    if(peserta.tglLahir != &quot;&quot;){
      var tgl = moment(&quot;&quot;+peserta.tglLahir+&quot;&quot;, &quot;DD-MM-YYYY&quot;).format(&quot;DD-MM-YYYY&quot;);
      var date = generateUmurByDate(tgl);
      $(&quot; , &quot;'&quot; , &quot;#umur_tahun&quot; , &quot;'&quot; , &quot;).val(date[&quot; , &quot;'&quot; , &quot;years&quot; , &quot;'&quot; , &quot;]);
      $(&quot; , &quot;'&quot; , &quot;#umur_bulan&quot; , &quot;'&quot; , &quot;).val(date[&quot; , &quot;'&quot; , &quot;months&quot; , &quot;'&quot; , &quot;]);
      $(&quot; , &quot;'&quot; , &quot;#umur_hari&quot; , &quot;'&quot; , &quot;).val(date[&quot; , &quot;'&quot; , &quot;days&quot; , &quot;'&quot; , &quot;]);
    }
}

function setAsuransi()
{
    var asuransi_id = $(&quot; , &quot;'&quot; , &quot;select[name=&quot;MPasien[asuransi_id]&quot;]&quot; , &quot;'&quot; , &quot;);
    var is_bridgingbpjs = asuransi_id.find(&quot; , &quot;'&quot; , &quot;:selected&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;is_bridgingbpjs&quot; , &quot;'&quot; , &quot;);
    var is_bridging_mandiri_inhealth = asuransi_id.find(&quot; , &quot;'&quot; , &quot;:selected&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;is_bridging_mandiri_inhealth&quot; , &quot;'&quot; , &quot;);
    var show_no_asuransi = asuransi_id.find(&quot; , &quot;'&quot; , &quot;:selected&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;show_no_asuransi&quot; , &quot;'&quot; , &quot;);
    var require_no_asuransi = asuransi_id.find(&quot; , &quot;'&quot; , &quot;:selected&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;require_no_asuransi&quot; , &quot;'&quot; , &quot;);

    $(&quot; , &quot;'&quot; , &quot;#nokainhealt&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;);
    $(&quot; , &quot;'&quot; , &quot;#asuransi_bpjs&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;);

    if(show_no_asuransi == &quot;0&quot;){
        $(&quot; , &quot;'&quot; , &quot;.no-asuransi&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;);
        $(&quot; , &quot;'&quot; , &quot;.no-asuransi input&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;);
    }else{
        $(&quot; , &quot;'&quot; , &quot;.no-asuransi&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;);
        if(require_no_asuransi == &quot;1&quot;){
            $(&quot; , &quot;'&quot; , &quot;.icon_asuransi&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;);
            $(&quot; , &quot;'&quot; , &quot;.no-asuransi input&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;, true);
        }else{
            $(&quot; , &quot;'&quot; , &quot;.icon_asuransi&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;);
            $(&quot; , &quot;'&quot; , &quot;.no-asuransi input&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;, false);
        }
        
        $(&quot; , &quot;'&quot; , &quot;#nokainhealt&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;, false);
        if(is_bridgingbpjs == &quot;1&quot;){
            $(&quot; , &quot;'&quot; , &quot;#data_peserta_bpjs&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;);
            $(&quot; , &quot;'&quot; , &quot;#button_bridgingbpjs&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;);
        }else{
            $(&quot; , &quot;'&quot; , &quot;#data_peserta_bpjs&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;);
            $(&quot; , &quot;'&quot; , &quot;#button_bridgingbpjs&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;);
            if (is_bridging_mandiri_inhealth == &quot;1&quot;) {
                if (isPenggunaMandiriInhealth == &quot;&quot; || isPenggunaMandiriInhealth == &quot;0&quot;) {
                    $(&quot; , &quot;'&quot; , &quot;.no-asuransi&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;);
                    $(&quot; , &quot;'&quot; , &quot;.no-asuransi input&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;);
                } else {
                    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[no_asuransi]&quot;]&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
                    $(&quot; , &quot;'&quot; , &quot;.no-asuransi input&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;, false);
                    if(require_no_asuransi == &quot;1&quot;) {
                        $(&quot; , &quot;'&quot; , &quot;#nokainhealt&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;, true);
                    }

                    $(&quot; , &quot;'&quot; , &quot;#asuransi_bpjs&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;);
                    $(&quot; , &quot;'&quot; , &quot;#nokainhealt&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;);
                }
            }
        }
    }
}

function checkAfter(){
    checkNoAsuransi();
    setTimeout(function() {
        setPesertaBpjs();
    }, 1000);
}

function setPesertaBpjs()
{
    var no_asuransi = $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[no_asuransi]&quot;]&quot; , &quot;'&quot; , &quot;);
    if(no_asuransi.val() == &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;){
        return false;
    }
    var nik = $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[nik]&quot;]&quot; , &quot;'&quot; , &quot;);
    if(no_asuransi.val() == &quot;&quot; &amp;&amp; nik.val() == &quot;&quot;){
        alert(&quot; , &quot;'&quot; , &quot;Silahkan isi &lt;b>No Asuransi&lt;/b> atau &lt;b>NIK&lt;/b> untuk mengecek kepesertaan BPJS!&quot; , &quot;'&quot; , &quot;);
        no_asuransi.parent().addClass(&quot; , &quot;'&quot; , &quot;has-error&quot; , &quot;'&quot; , &quot;);
        nik.parent().addClass(&quot; , &quot;'&quot; , &quot;has-error&quot; , &quot;'&quot; , &quot;);
        return false;
    }else{
        no_asuransi.parent().removeClass(&quot; , &quot;'&quot; , &quot;has-error&quot; , &quot;'&quot; , &quot;);
        search = no_asuransi.val();
        if(search == &quot;&quot;){
            search = nik.val();
        }
    }
    no_asuransi.addClass(&quot; , &quot;'&quot; , &quot;ui-autocomplete-loading&quot; , &quot;'&quot; , &quot;);
    var asuransi_id = $(&quot; , &quot;'&quot; , &quot;select[name=&quot;MPasien[asuransi_id]&quot;]&quot; , &quot;'&quot; , &quot;).val();

    $.ajax({
        url: &quot;https://demo.epuskesmas.id/pasien/create&quot;,
        method: &quot;GET&quot;,
        dataType: &quot;json&quot;,
        data: {
            api: &quot; , &quot;'&quot; , &quot;pesertabpjs&quot; , &quot;'&quot; , &quot;,
            search: search,
            asuransi_id: asuransi_id,
        },
        success: function(data) {
            let is_bridgingbpjs_nonactive = data.is_bridgingbpjs_nonactive ?? &quot;false&quot;;
            if (is_bridgingbpjs_nonactive == &quot;active&quot;){

                alert(&quot; , &quot;'&quot; , &quot;Mohon Maaf Settingan Sinkron Asuransi Tersebut sedang Non-aktif Tidak Bisa melakukan Pengecekan No. Asuransi&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;warning&quot; , &quot;'&quot; , &quot;)
                return false;
            }

            no_asuransi.removeClass(&quot; , &quot;'&quot; , &quot;ui-autocomplete-loading&quot; , &quot;'&quot; , &quot;);
            if(data.hasOwnProperty(&quot; , &quot;'&quot; , &quot;metaData&quot; , &quot;'&quot; , &quot;)){
                if(data.metaData.code == 200){
                    peserta = data.response;
                    if(peserta.noKTP == null){
                        alert(&quot;&lt;b>No. KTP (NIK)&lt;\/b> peserta BPJS ini bernilai &lt;b>null&lt;\/b> di response Web Service.&quot; 
                            +&quot;&lt;br> 1. Pastikan pasien BPJS ini tidak memiliki kartu / nomor &lt;b>peserta BPJS ganda&lt;\/b> dan sudah melunasi pembayaran &lt;b>premi&lt;\/b> bulan ini.&quot;
                            +&quot;&lt;br> 2. Infokan kepada pasien BPJS ini agar melakukan cek dan pembaharuan &lt;b>data peserta&lt;\/b> BPJS ke kantor BPJS.&quot;
                            +&quot;&lt;br> 3. Anda tetap bisa mendaftarkan pasien BPJS ini sebagai pasien, namun pastikan untuk mengecek &lt;b>status bpjs&lt;\/b> di aplikasi ePuskesmas dan &lt;b>mengecek data di PCare&lt;\/b>.&quot;
                            +&quot;&lt;br> 4. Info selengkapnya silahkan hubungi &lt;b>petugas BPJS&lt;\/b>.&quot;, &quot;warning&quot;, 0);
                    }
                    alert(&quot;Data peserta BPJS ditemukan!&quot;, &quot;success&quot;);
                    setDataDariApiBpjs(peserta);
                    no_asuransi.parent().removeClass(&quot; , &quot;'&quot; , &quot;has-error&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;has-success&quot; , &quot;'&quot; , &quot;);
                    if($(&quot;#normcheck&quot;).val() == &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;){
                        $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[nik]&quot;]&quot; , &quot;'&quot; , &quot;).trigger(&quot; , &quot;'&quot; , &quot;blur&quot; , &quot;'&quot; , &quot;);
                        if($(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[nik]&quot;]&quot; , &quot;'&quot; , &quot;).val() != &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;){
                            alert(&quot; , &quot;'&quot; , &quot;Pengecekan data pasien di sistem selesai, silahkan cek kembali isian Anda  ..&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;success&quot; , &quot;'&quot; , &quot;);
                        }
                    }
                }else{
                    if(data.metaData.code == 401){
                        alert(&quot;Anda tidak diperkenankan terkoneksi ke Webservice BPJS. Pastikan username dan password di &lt;b>Pengaturan - BPJS&lt;\/b> disesuaikan dengan username dan password PCare terbaru!&lt;br> &lt;span class=&quot; , &quot;'&quot; , &quot;text-danger&quot; , &quot;'&quot; , &quot;> Kode Error: &lt;b>&quot;+data.metaData.code+&quot;&lt;\/b>&lt;\/span>&quot;, &quot;warning&quot;);
                    }else{
                        alert(&quot;Data peserta BPJS tidak ditemukan! Cek kembali &lt;b>No Asuransi&lt;\/b> yang diinput &lt;b>atau&lt;\/b> tunggu beberapa saat hingga Webservice BPJS kembali normal.&lt;br> &lt;span class=&quot; , &quot;'&quot; , &quot;text-danger&quot; , &quot;'&quot; , &quot;> Kode Error: &lt;b>&quot;+data.metaData.code+&quot;&lt;\/b>&lt;\/span>&quot;, &quot;warning&quot;);
                    }
                    no_asuransi.parent().addClass(&quot; , &quot;'&quot; , &quot;has-error&quot; , &quot;'&quot; , &quot;);
                    nik.parent().addClass(&quot; , &quot;'&quot; , &quot;has-error&quot; , &quot;'&quot; , &quot;);
                    $(&quot; , &quot;'&quot; , &quot;#data_peserta_bpjs&quot; , &quot;'&quot; , &quot;).html(&quot;&quot;);
                }
            }else{
                if(data.hasOwnProperty(&quot; , &quot;'&quot; , &quot;o2o&quot; , &quot;'&quot; , &quot;)){
                    alert(data.o2o.message, data.o2o.status);
                }else{
                    alert(&quot;Data peserta BPJS tidak ditemukan! Cek kembali &lt;b>No Asuransi&lt;\/b> yang diinput &lt;b>&quot;, &quot;warning&quot;);
                }
                no_asuransi.parent().addClass(&quot; , &quot;'&quot; , &quot;has-error&quot; , &quot;'&quot; , &quot;);
                nik.parent().addClass(&quot; , &quot;'&quot; , &quot;has-error&quot; , &quot;'&quot; , &quot;);
                $(&quot; , &quot;'&quot; , &quot;#data_peserta_bpjs&quot; , &quot;'&quot; , &quot;).html(&quot;&quot;);
            }
        },
        error: function (xhr, ajaxOptions, thrownError) {
            console.log(xhr.status);
            console.log(thrownError);
            no_asuransi.removeClass(&quot; , &quot;'&quot; , &quot;ui-autocomplete-loading&quot; , &quot;'&quot; , &quot;);
        }
    });
}

function setRequiredForm(reset = false, show = 0)
{
    var requiredInput = [
                        &quot; , &quot;'&quot; , &quot;nik&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;tempat_lahir&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;tanggal_lahir&quot; , &quot;'&quot; , &quot;,
                        &quot; , &quot;'&quot; , &quot;propinsi_id&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;kota_id&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;kecamatan_id&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;kelurahan_id&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;alamat&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;rt&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;rw&quot; , &quot;'&quot; , &quot;,
                        &quot; , &quot;'&quot; , &quot;pekerjaan_id&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;agama&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;pendidikan&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;status_perkawinan&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;status_keluarga&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;warganegara&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;no_hp&quot; , &quot;'&quot; , &quot;
                        ];

    if(global_required!=&quot; , &quot;'&quot; , &quot;undefined&quot; , &quot;'&quot; , &quot;){
        global_required.forEach(function(item,index){
            for (var i=0; i&lt;requiredInput.length; i++)
                if (requiredInput[i] === item){
                    delete requiredInput[i]; 
                }                                    
        });
    }

    if(reset == false){
        requiredInput.forEach(function(item,index){
            var obj = $(&quot; , &quot;'&quot; , &quot;[name=&quot;MPasien[&quot; , &quot;'&quot; , &quot;+item+&quot; , &quot;'&quot; , &quot;]&quot;]&quot; , &quot;'&quot; , &quot;);
            obj.attr(&quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,true);
            obj.parents(&quot; , &quot;'&quot; , &quot;.form-group&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;span[style]&quot; , &quot;'&quot; , &quot;).remove();
            obj.parents(&quot; , &quot;'&quot; , &quot;.form-group&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;label&quot; , &quot;'&quot; , &quot;).append(&quot; , &quot;'&quot; , &quot; &lt;span style=&quot;color:red;&quot;>*&lt;/span>&quot; , &quot;'&quot; , &quot;);
        });    
    }else{
        requiredInput.forEach(function(item,index){
            var obj = $(&quot; , &quot;'&quot; , &quot;[name=&quot;MPasien[&quot; , &quot;'&quot; , &quot;+item+&quot; , &quot;'&quot; , &quot;]&quot;]&quot; , &quot;'&quot; , &quot;);            
            obj.removeAttr(&quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;);
            obj.parents(&quot; , &quot;'&quot; , &quot;.form-group&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;span[style]&quot; , &quot;'&quot; , &quot;).remove();
        });
    }
    
}

function setRequiredFormKtp(reset = false)
{
    var requiredInput = [
                        &quot; , &quot;'&quot; , &quot;ktp_propinsi_id&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;ktp_kota_id&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;ktp_kecamatan_id&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;ktp_kelurahan_id&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;ktp_alamat&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;ktp_rt&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;ktp_rw&quot; , &quot;'&quot; , &quot;
                        ];
    set_up = $(&quot; , &quot;'&quot; , &quot;[name=&quot;MPasien[&quot; , &quot;'&quot; , &quot;+requiredInput[0]+&quot; , &quot;'&quot; , &quot;]&quot;]&quot; , &quot;'&quot; , &quot;).is(&quot; , &quot;'&quot; , &quot;[required]&quot; , &quot;'&quot; , &quot;);
    if(reset==true){
        $(&quot; , &quot;'&quot; , &quot;#collapseAlamatKtp.collapse&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;in&quot; , &quot;'&quot; , &quot;);
    }
    if(set_up == false &amp;&amp; reset == false){
        requiredInput.forEach(function(item,index){
            var obj = $(&quot; , &quot;'&quot; , &quot;[name=&quot;MPasien[&quot; , &quot;'&quot; , &quot;+item+&quot; , &quot;'&quot; , &quot;]&quot;]&quot; , &quot;'&quot; , &quot;);
            obj.attr(&quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,true);
            obj.parents(&quot; , &quot;'&quot; , &quot;.form-group&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;span[style]&quot; , &quot;'&quot; , &quot;).remove();
            obj.parents(&quot; , &quot;'&quot; , &quot;.form-group&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;label&quot; , &quot;'&quot; , &quot;).append(&quot; , &quot;'&quot; , &quot; &lt;span style=&quot;color:red;&quot;>*&lt;/span>&quot; , &quot;'&quot; , &quot;);
        });    
    }else{
        $(&quot; , &quot;'&quot; , &quot;[name=&quot;MPasien[ktp_alamat_berbeda]&quot;]&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;checked&quot; , &quot;'&quot; , &quot;);
        requiredInput.forEach(function(item,index){
            var obj = $(&quot; , &quot;'&quot; , &quot;[name=&quot;MPasien[&quot; , &quot;'&quot; , &quot;+item+&quot; , &quot;'&quot; , &quot;]&quot;]&quot; , &quot;'&quot; , &quot;);
            obj.removeAttr(&quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;);
            obj.parents(&quot; , &quot;'&quot; , &quot;.form-group&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;span[style]&quot; , &quot;'&quot; , &quot;).remove();
        });
    }
}

let isPenggunaMandiriInhealth = &quot;&quot;;

$(&quot;form[form-ajax-print]&quot;).each(function(){
    $(this).on(&quot; , &quot;'&quot; , &quot;submit&quot; , &quot;'&quot; , &quot;,function(event){
        event.preventDefault();
        var button_submit = $(this).find(&quot; , &quot;'&quot; , &quot;button[type=submit]&quot; , &quot;'&quot; , &quot;);
        var button_print = $(this).find(&quot; , &quot;'&quot; , &quot;button[id=button_print]&quot; , &quot;'&quot; , &quot;);
        var button_print_kartu = $(this).find(&quot; , &quot;'&quot; , &quot;button[id=button_print_kartu]&quot; , &quot;'&quot; , &quot;);
        var button_reset = $(this).find(&quot; , &quot;'&quot; , &quot;button[type=reset]&quot; , &quot;'&quot; , &quot;);
        button_submit.addClass(&quot; , &quot;'&quot; , &quot;loading&quot; , &quot;'&quot; , &quot;);

        if(isPenggunaMandiriInhealth == &quot;1&quot;) 
        {
            var is_bridging_mandiri_inhealth = $(&quot; , &quot;'&quot; , &quot;select[name=&quot;MPasien[asuransi_id]&quot;]&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;:selected&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;is_bridging_mandiri_inhealth&quot; , &quot;'&quot; , &quot;);

            if (is_bridging_mandiri_inhealth == &quot;1&quot;) {
                let nokainhealt = $(&quot; , &quot;'&quot; , &quot;input[name=&quot;nokainhealt&quot;]&quot; , &quot;'&quot; , &quot;).val();
                $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[no_asuransi]&quot;]&quot; , &quot;'&quot; , &quot;).val(nokainhealt);                
            }

        }

        $(&quot; , &quot;'&quot; , &quot;#button_saves&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;loading&quot; , &quot;'&quot; , &quot;);
        for (instance in CKEDITOR.instances) {
            CKEDITOR.instances[instance].updateElement();
        }
                
        unsetSeparatorNumber();
        $.ajax({
            url: $(this).attr(&quot; , &quot;'&quot; , &quot;action&quot; , &quot;'&quot; , &quot;),
            method: $(this).attr(&quot; , &quot;'&quot; , &quot;method&quot; , &quot;'&quot; , &quot;),
            dataType: &quot;json&quot;,
            data: $(this).serialize(),
            success: function(data) {
                if(data.activeSwal == &quot;true&quot;){
                    button_submit.removeClass(&quot; , &quot;'&quot; , &quot;loading&quot; , &quot;'&quot; , &quot;);
                    $(&quot;#button_saves&quot;).removeClass(&quot; , &quot;'&quot; , &quot;loading&quot; , &quot;'&quot; , &quot;);
                    Swal.fire({ 
                      title: &quot; , &quot;'&quot; , &quot;Apakah Anda Yakin Merubah Data ?&quot; , &quot;'&quot; , &quot;,
                      type: &quot; , &quot;'&quot; , &quot;info&quot; , &quot;'&quot; , &quot;,
                      customClass: &quot; , &quot;'&quot; , &quot;swal-wide-pasien&quot; , &quot;'&quot; , &quot;,
                      showCancelButton: true,
                      confirmButtonColor: &quot; , &quot;'&quot; , &quot;#3085d6&quot; , &quot;'&quot; , &quot;,
                      cancelButtonColor: &quot; , &quot;'&quot; , &quot;#d33&quot; , &quot;'&quot; , &quot;,
                      confirmButtonText: &quot; , &quot;'&quot; , &quot;Ya&quot; , &quot;'&quot; , &quot;,
                      cancelButtonText: &quot; , &quot;'&quot; , &quot;Tidak&quot; , &quot;'&quot; , &quot;,
                      html: data.message,
                    }).then((result) => {
                      if (result.value) {
                        $(&quot;#cekBpjs&quot;).val(&quot;false&quot;);
                        $(&quot;#button_save&quot;).click();
                      }else if (result.dismiss == &quot;cancel&quot;){
                        $(&quot;#cekBpjs&quot;).val(&quot;default&quot;);
                      }else{
                        $(&quot;#cekBpjs&quot;).val(&quot;default&quot;);
                      }
                      
                    });
                    return false;
                }

                alert(data.message, data.status);
                button_submit.removeClass(&quot; , &quot;'&quot; , &quot;loading&quot; , &quot;'&quot; , &quot;);
                $(&quot; , &quot;'&quot; , &quot;#button_saves&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;loading&quot; , &quot;'&quot; , &quot;);
                if(data.status == &quot; , &quot;'&quot; , &quot;success&quot; , &quot;'&quot; , &quot;){
                    syncPasienKiosk();
                    button_submit.hide();
                    $(&quot; , &quot;'&quot; , &quot;#button_saves&quot; , &quot;'&quot; , &quot;).hide();
                    button_print.show();
                    button_print_kartu.show();
                    button_reset.click(function(){
                        button_submit.show();
                        $(&quot; , &quot;'&quot; , &quot;#button_saves&quot; , &quot;'&quot; , &quot;).show();
                        button_print.hide();
                        button_print.attr(&quot; , &quot;'&quot; , &quot;onclick&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
                        button_print_kartu.hide();
                        button_print_kartu.attr(&quot; , &quot;'&quot; , &quot;onclick&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
                        $( &quot;label[id=created_id]&quot; ).html(&quot; , &quot;'&quot; , &quot;- Otomatis -&quot; , &quot;'&quot; , &quot;);
                    });
                    if(data.id != undefined &amp;&amp; data.id != &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;){
                        $( &quot;label[id=created_id]&quot; ).html(data.id);
                    }
                    if(data.next_url != undefined &amp;&amp; data.next_url != &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;){
                        $(&quot; , &quot;'&quot; , &quot;#button_next&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;href&quot; , &quot;'&quot; , &quot;,data.next_url);
                        $(&quot; , &quot;'&quot; , &quot;#button_next&quot; , &quot;'&quot; , &quot;).show();
                    }
                    if(data.print_url != undefined &amp;&amp; data.print_url != &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;){
                        button_print.attr(&quot; , &quot;'&quot; , &quot;onclick&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;window.open(&quot;&quot; , &quot;'&quot; , &quot;+data.print_url+&quot; , &quot;'&quot; , &quot;&quot;, &quot;&quot;, &quot;left=100,top=100&quot;);&quot; , &quot;'&quot; , &quot;);
                    }

                    if($(&quot; , &quot;'&quot; , &quot;#askep_id&quot; , &quot;'&quot; , &quot;) != null){
                        $(&quot; , &quot;'&quot; , &quot;#button_askep&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;href&quot; , &quot;'&quot; , &quot;, data.next_url+&quot; , &quot;'&quot; , &quot;?pengkajian_keperawatan_id=&quot; , &quot;'&quot; , &quot;+$(&quot; , &quot;'&quot; , &quot;#askep_id&quot; , &quot;'&quot; , &quot;).val());
                    }

                    if(data.print_url_kartu != undefined &amp;&amp; data.print_url_kartu != &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;){
                        button_print_kartu.attr(&quot; , &quot;'&quot; , &quot;onclick&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;window.open(&quot;&quot; , &quot;'&quot; , &quot;+data.print_url_kartu+&quot; , &quot;'&quot; , &quot;&quot;, &quot;&quot;, &quot;left=100,top=100,width=420,height=260&quot;);&quot; , &quot;'&quot; , &quot;);
                    }

                    if(data.show_url != undefined &amp;&amp; data.show_url != &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;){
                        var url = data.show_url;
                        $.pjax({url: url, container: &quot; , &quot;'&quot; , &quot;#content&quot; , &quot;'&quot; , &quot;});
                    }
                }
                setSeparatorNumber();
            },
            error: function (xhr, ajaxOptions, thrownError) {
                alert(&quot;Terjadi kesalahan sistem, silahkan hubungi team support kami!&quot;, &quot;danger&quot;);
                button_submit.removeClass(&quot; , &quot;'&quot; , &quot;loading&quot; , &quot;'&quot; , &quot;);
                $(&quot; , &quot;'&quot; , &quot;#button_saves&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;loading&quot; , &quot;'&quot; , &quot;);
                console.log(xhr.status);
                console.log(thrownError);
                setSeparatorNumber();
            }
        });
    });
});

function checkNoAsuransi(){
    checkAsuransiVal();
    var no_asuransi = $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[no_asuransi]&quot;]&quot; , &quot;'&quot; , &quot;).val();
    var asuransi_id = $(&quot; , &quot;'&quot; , &quot;select[name=&quot;MPasien[asuransi_id]&quot;]&quot; , &quot;'&quot; , &quot;).val();
    $.ajax({
        url: &quot;https://demo.epuskesmas.id/pasien/create&quot;,
        method: &quot;GET&quot;,
        dataType: &quot;json&quot;,
        data: {
            check: &quot; , &quot;'&quot; , &quot;no_asuransi&quot; , &quot;'&quot; , &quot;,
            search: {
                &quot; , &quot;'&quot; , &quot;no_asuransi&quot; , &quot;'&quot; , &quot;:no_asuransi,
                &quot; , &quot;'&quot; , &quot;asuransi_id&quot; , &quot;'&quot; , &quot;:asuransi_id,
                &quot; , &quot;'&quot; , &quot;id&quot; , &quot;'&quot; , &quot;:&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;
            }
        },
        success: function(data) {
            if(data.status == &quot; , &quot;'&quot; , &quot;success&quot; , &quot;'&quot; , &quot;){
                $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[no_asuransi]&quot;]&quot; , &quot;'&quot; , &quot;).parent().addClass(&quot; , &quot;'&quot; , &quot;has-success&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;has-error&quot; , &quot;'&quot; , &quot;);
            }else if(data.length &lt;= 0){
                $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[no_asuransi]&quot;]&quot; , &quot;'&quot; , &quot;).parent().addClass(&quot; , &quot;'&quot; , &quot;has-error&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;has-success&quot; , &quot;'&quot; , &quot;);
            }else{
                alert(data.message, data.status);
                $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[no_asuransi]&quot;]&quot; , &quot;'&quot; , &quot;).parent().addClass(&quot; , &quot;'&quot; , &quot;has-error&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;has-success&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;);
                $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[no_asuransi]&quot;]&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
                $(&quot; , &quot;'&quot; , &quot;#data_peserta_bpjs&quot; , &quot;'&quot; , &quot;).html(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
            }
            
        },
        error: function (xhr, ajaxOptions, thrownError) {
            console.log(xhr.status);
            console.log(thrownError);
            $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[no_asuransi]&quot;]&quot; , &quot;'&quot; , &quot;).parent().removeClass(&quot; , &quot;'&quot; , &quot;has-success&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;has-error&quot; , &quot;'&quot; , &quot;);
        }
    });

}

function checkNik(nik){
    if(&quot;e-Puskesmas&quot; == &quot; , &quot;'&quot; , &quot;e-Clinic&quot; , &quot;'&quot; , &quot;) {
      var Nik = $(&quot;input[name=&quot; , &quot;'&quot; , &quot;MPasien[nik]&quot; , &quot;'&quot; , &quot;]&quot;);
      Nik.attr(&quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;, true); 
      Nik.parents(&quot; , &quot;'&quot; , &quot;.form-group&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;span[style]&quot; , &quot;'&quot; , &quot;).remove();
      Nik.parents(&quot; , &quot;'&quot; , &quot;.form-group&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;label&quot; , &quot;'&quot; , &quot;).append(&quot; , &quot;'&quot; , &quot; &lt;span style=&quot;color:red;&quot;>*&lt;/span>&quot; , &quot;'&quot; , &quot;);
      var radio = $(&quot; , &quot;'&quot; , &quot;#radioNik&quot; , &quot;'&quot; , &quot;);
      radio.attr(&quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;, false);
      radio.prop(&quot; , &quot;'&quot; , &quot;checked&quot; , &quot;'&quot; , &quot;, false);
      radio.attr(&quot; , &quot;'&quot; , &quot;checked&quot; , &quot;'&quot; , &quot;, false);
      radio.parent(&quot; , &quot;'&quot; , &quot;label&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;span[style]&quot; , &quot;'&quot; , &quot;).remove();
    }
    if(nik.length != 16){
        var message = &quot;&quot;;
        if(nik.length &lt; 16){
            message += &quot;NIK kurang dari 16 digit!&quot;;
        }else{
            message += &quot;NIK lebih dari 16 digit!&quot;;
        }
        message +=&quot;&lt;br>Silahkan periksa kembali dan pastikan NIK yang diinput sesuai dengan yang tercantum di KTP atau KK!&quot;;
        alert(message);
        $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[nik]&quot;]&quot; , &quot;'&quot; , &quot;).parent().addClass(&quot; , &quot;'&quot; , &quot;has-error&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;has-success&quot; , &quot;'&quot; , &quot;);
        return false;
    }
        $.ajax({
        url: &quot;https://demo.epuskesmas.id/pasien/create&quot;,
        method: &quot;GET&quot;,
        dataType: &quot;json&quot;,
        data: {
            check: &quot; , &quot;'&quot; , &quot;nik&quot; , &quot;'&quot; , &quot;,
            search: {
                &quot; , &quot;'&quot; , &quot;nik&quot; , &quot;'&quot; , &quot;:nik,
                &quot; , &quot;'&quot; , &quot;id&quot; , &quot;'&quot; , &quot;:&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;
            }
        },
        success: function(data) {
                            if(data.puskesmas == true){
                    window.location = &quot;https://demo.epuskesmas.id/pasien/edit/&quot;+data.result.id;
                }else{
                    if(data.result){
                        setDataDariKota(data.result);
                    }
                }
                return false;
                        if(data.status == &quot; , &quot;'&quot; , &quot;success&quot; , &quot;'&quot; , &quot;){
                $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[nik]&quot;]&quot; , &quot;'&quot; , &quot;).parent().addClass(&quot; , &quot;'&quot; , &quot;has-success&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;has-error&quot; , &quot;'&quot; , &quot;);
            }else{
                alert(data.message, data.status);
                $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[nik]&quot;]&quot; , &quot;'&quot; , &quot;).parent().addClass(&quot; , &quot;'&quot; , &quot;has-error&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;has-success&quot; , &quot;'&quot; , &quot;);
            }
        },
        error: function (xhr, ajaxOptions, thrownError) {
            console.log(xhr.status);
            console.log(thrownError);
            $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[nik]&quot;]&quot; , &quot;'&quot; , &quot;).parent().removeClass(&quot; , &quot;'&quot; , &quot;has-success&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;has-error&quot; , &quot;'&quot; , &quot;);
        }
    });
}

function getDisdukcapil(){
    var nik = $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[nik]&quot;]&quot; , &quot;'&quot; , &quot;);
    $.ajax({
        url: &quot;https://demo.epuskesmas.id/pasien/create&quot;,
        method: &quot;GET&quot;,
        dataType: &quot;json&quot;,
        data: {
            getData: &quot; , &quot;'&quot; , &quot;disdukcapil&quot; , &quot;'&quot; , &quot;,
            search: {
                &quot; , &quot;'&quot; , &quot;nik&quot; , &quot;'&quot; , &quot;:nik.val(),
            }
        },
        success: function(data) {
            setRequiredForm(true);
            $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[verified_by]&quot;]&quot; , &quot;'&quot; , &quot;).prop(&quot;checked&quot;, false);
            if(data.kesalahan){
                alert(data.kesalahan);
                $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[nik]&quot;]&quot; , &quot;'&quot; , &quot;).parent().addClass(&quot; , &quot;'&quot; , &quot;has-error&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;has-success&quot; , &quot;'&quot; , &quot;);
            }else{
                alert(&quot;Biodata Disdukcapil ditemukan!&quot;, &quot;success&quot;);
                setRequiredForm(false);
                $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[verified_by]&quot;]&quot; , &quot;'&quot; , &quot;).prop(&quot;checked&quot;, true);
                setDataDisdukcapil(data);
                $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[nik]&quot;]&quot; , &quot;'&quot; , &quot;).parent().addClass(&quot; , &quot;'&quot; , &quot;has-success&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;has-error&quot; , &quot;'&quot; , &quot;);
            }
        },
        error: function (xhr, ajaxOptions, thrownError) {
            console.log(xhr.status);
            console.log(thrownError);
            $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[nik]&quot;]&quot; , &quot;'&quot; , &quot;).parent().removeClass(&quot; , &quot;'&quot; , &quot;has-success&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;has-error&quot; , &quot;'&quot; , &quot;);
        }
    });
}


function saveAtas(){
    $(&quot; , &quot;'&quot; , &quot;#button_saves&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;loading&quot; , &quot;'&quot; , &quot;);
    $(&quot; , &quot;'&quot; , &quot;#button_save&quot; , &quot;'&quot; , &quot;).click();
}

function setDataDisdukcapil(data)
{
    setWilayah(data);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[nama]&quot;]&quot; , &quot;'&quot; , &quot;).val(data.nama);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[jenis_kelamin]&quot;][value=&quot;&quot; , &quot;'&quot; , &quot;+data.jenis_kelamin+&quot; , &quot;'&quot; , &quot;&quot;]&quot; , &quot;'&quot; , &quot;).prop(&quot;checked&quot;,true);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[tanggal_lahir]&quot;]&quot; , &quot;'&quot; , &quot;).val(data.tanggal_lahir);
    $(&quot; , &quot;'&quot; , &quot;select[name=&quot;MPasien[gol_darah]&quot;]&quot; , &quot;'&quot; , &quot;).val(data.gol_darah);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[no_hp]&quot;]&quot; , &quot;'&quot; , &quot;).val(data.no_hp);
    $(&quot; , &quot;'&quot; , &quot;select[name=&quot;MPasien[status_keluarga]&quot;]&quot; , &quot;'&quot; , &quot;).val(data.status_keluarga);
    $(&quot; , &quot;'&quot; , &quot;select[name=&quot;MPasien[status_perkawinan]&quot;]&quot; , &quot;'&quot; , &quot;).val(data.status_perkawinan);
    $(&quot; , &quot;'&quot; , &quot;select[name=&quot;MPasien[pendidikan]&quot;]&quot; , &quot;'&quot; , &quot;).val(data.pendidikan);
    $(&quot; , &quot;'&quot; , &quot;select[name=&quot;MPasien[agama]&quot;]&quot; , &quot;'&quot; , &quot;).val(data.agama);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;pekerjaan_nama&quot;]&quot; , &quot;'&quot; , &quot;).val(data.pekerjaan_nama);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[pekerjaan_id]&quot;]&quot; , &quot;'&quot; , &quot;).val(data.pekerjaan_id);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[instansi]&quot;]&quot; , &quot;'&quot; , &quot;).val(data.instansi);
    $(&quot; , &quot;'&quot; , &quot;textarea[name=&quot;MPasien[alamat]&quot;]&quot; , &quot;'&quot; , &quot;).val(data.alamat);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[rt]&quot;]&quot; , &quot;'&quot; , &quot;).val(data.rt);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[rw]&quot;]&quot; , &quot;'&quot; , &quot;).val(data.rw);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[nama_ayah]&quot;]&quot; , &quot;'&quot; , &quot;).val(data.nama_ayah);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[nama_ibu]&quot;]&quot; , &quot;'&quot; , &quot;).val(data.nama_ibu);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[tempat_lahir]&quot;]&quot; , &quot;'&quot; , &quot;).val(data.tempat_lahir);
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[no_kk]&quot;]&quot; , &quot;'&quot; , &quot;).val(data.no_kk);
    $(&quot; , &quot;'&quot; , &quot;select[name=&quot;MPasien[warganegara]&quot;]&quot; , &quot;'&quot; , &quot;).val(data.warganegara);
}

    var global_required = [&quot;&quot;] ;


$(document).ready(function(){

        setAsuransi();


    if($(&quot; , &quot;'&quot; , &quot;#checked_verifikasi&quot; , &quot;'&quot; , &quot;).is(&quot; , &quot;'&quot; , &quot;:checked&quot; , &quot;'&quot; , &quot;)) {
        setRequiredForm(false);
    }else{
        setRequiredForm(true);
    }

    $(&quot;input[name=&quot; , &quot;'&quot; , &quot;MPasien[no_dok_rm]&quot; , &quot;'&quot; , &quot;]&quot;).on(&quot; , &quot;'&quot; , &quot;blur&quot; , &quot;'&quot; , &quot;, function () {
        $.ajax({
            url: &quot;https://demo.epuskesmas.id/pasien/create&quot;,
            method: &quot;GET&quot;,
            dataType: &quot;json&quot;,
            data: {
                no_dok_rm: this.value
            },
            success: function(data) {
                if (data.message != &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;) {
                    Swal.fire({ 
                        title: &quot; , &quot;'&quot; , &quot;Perhatian!&quot; , &quot;'&quot; , &quot;,
                        type: &quot; , &quot;'&quot; , &quot;info&quot; , &quot;'&quot; , &quot;,
                        customClass: &quot; , &quot;'&quot; , &quot;swal-wide-pasien&quot; , &quot;'&quot; , &quot;,
                        showCancelButton: true,
                        confirmButtonColor: &quot; , &quot;'&quot; , &quot;#3085d6&quot; , &quot;'&quot; , &quot;,
                        cancelButtonColor: &quot; , &quot;'&quot; , &quot;#d33&quot; , &quot;'&quot; , &quot;,
                        confirmButtonText: &quot; , &quot;'&quot; , &quot;Ya&quot; , &quot;'&quot; , &quot;,
                        cancelButtonText: &quot; , &quot;'&quot; , &quot;Tidak&quot; , &quot;'&quot; , &quot;,
                        html: data.message,
                    }).then((result) => {
                        if (result.value) {
                            $(&quot;input[name=&quot; , &quot;'&quot; , &quot;MPasien[no_dok_rm]&quot; , &quot;'&quot; , &quot;]&quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;).focus();
                        }
                    });
                }
            },
            error: function (xhr, ajaxOptions, thrownError) {
                console.log(xhr.status);
                console.log(thrownError);
            }
        });
    });
});

$(&quot; , &quot;'&quot; , &quot;#checked_verifikasi&quot; , &quot;'&quot; , &quot;).click( function(){
    if($(this).is(&quot; , &quot;'&quot; , &quot;:checked&quot; , &quot;'&quot; , &quot;)) {
        setRequiredForm(false);
    }else{
        setRequiredForm(true);
    }
});

function checkAsuransiVal(){
    var asuransi_id = $(&quot; , &quot;'&quot; , &quot;select[name=&quot;MPasien[asuransi_id]&quot;]&quot; , &quot;'&quot; , &quot;);
    var is_bridgingbpjs = asuransi_id.find(&quot; , &quot;'&quot; , &quot;:selected&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;is_bridgingbpjs&quot; , &quot;'&quot; , &quot;);
    var no_asuransi = $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[no_asuransi]&quot;]&quot; , &quot;'&quot; , &quot;).val();
    var pad = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
    if (is_bridgingbpjs == &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot; &amp;&amp; no_asuransi.length &lt; 13 &amp;&amp; no_asuransi != &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;) {
        for (i=1; i &lt;= (13 - no_asuransi.length); i++) {
            pad += &quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;;
        }
    }
    $(&quot; , &quot;'&quot; , &quot;input[name=&quot;MPasien[no_asuransi]&quot;]&quot; , &quot;'&quot; , &quot;).val(`${pad}${no_asuransi}`);
}

function printLabel(id, tipe) {
    window.open(`https://demo.epuskesmas.id/pasien/printlabel/${id}/${tipe}`, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;left=0,top=0&quot; , &quot;'&quot; , &quot;);
}

$(&quot;select.warganegara&quot;).change(function(){
        var selected = $(this).children(&quot;option:selected&quot;).val();
        if (selected == &quot;INDONESIA&quot;) {
            $(&quot;input[name=&quot; , &quot;'&quot; , &quot;MPasien[negara_asal]&quot; , &quot;'&quot; , &quot;]&quot;).val(&quot; , &quot;'&quot; , &quot;INDONESIA&quot; , &quot;'&quot; , &quot;);
            if (&quot;e-Puskesmas&quot; == &quot; , &quot;'&quot; , &quot;e-Clinic&quot; , &quot;'&quot; , &quot;) {
              var paspor = $(&quot;input[name=&quot; , &quot;'&quot; , &quot;MPasien[no_paspor]&quot; , &quot;'&quot; , &quot;]&quot;);
              paspor.attr(&quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;, false);
              paspor.parents(&quot; , &quot;'&quot; , &quot;.form-group&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;span[style]&quot; , &quot;'&quot; , &quot;).remove();
              paspor.parents(&quot; , &quot;'&quot; , &quot;.form-group&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;);
              var nik = $(&quot;input[name=&quot; , &quot;'&quot; , &quot;MPasien[nik]&quot; , &quot;'&quot; , &quot;]&quot;);
              nik.parents(&quot; , &quot;'&quot; , &quot;.form-group&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;);
              nik.attr(&quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;, true);
              nik.parents(&quot; , &quot;'&quot; , &quot;.form-group&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;span[style]&quot; , &quot;'&quot; , &quot;).remove();
              nik.parents(&quot; , &quot;'&quot; , &quot;.form-group&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;label&quot; , &quot;'&quot; , &quot;).append(&quot; , &quot;'&quot; , &quot; &lt;span style=&quot;color:red;&quot;>*&lt;/span>&quot; , &quot;'&quot; , &quot;);
              var radnik = $(&quot; , &quot;'&quot; , &quot;#radioNik&quot; , &quot;'&quot; , &quot;);
              radnik.parents(&quot; , &quot;'&quot; , &quot;.form-group&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;);
              setRequiredFormAlamat(true);
            }
        } else if (selected == &quot;ASING&quot;){
            $(&quot;input[name=&quot; , &quot;'&quot; , &quot;MPasien[negara_asal]&quot; , &quot;'&quot; , &quot;]&quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
            if (&quot;e-Puskesmas&quot; == &quot; , &quot;'&quot; , &quot;e-Clinic&quot; , &quot;'&quot; , &quot;) {
              var paspor = $(&quot;input[name=&quot; , &quot;'&quot; , &quot;MPasien[no_paspor]&quot; , &quot;'&quot; , &quot;]&quot;);
              paspor.parents(&quot; , &quot;'&quot; , &quot;.form-group&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;);
              paspor.parents(&quot; , &quot;'&quot; , &quot;.form-group&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;label&quot; , &quot;'&quot; , &quot;).append(&quot; , &quot;'&quot; , &quot; &lt;span style=&quot;color:red;&quot;>*&lt;/span>&quot; , &quot;'&quot; , &quot;);
              paspor.attr(&quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;, true);
              var nik = $(&quot;input[name=&quot; , &quot;'&quot; , &quot;MPasien[nik]&quot; , &quot;'&quot; , &quot;]&quot;);
              nik.val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
              nik.attr(&quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;, false);
              nik.parents(&quot; , &quot;'&quot; , &quot;.form-group&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;span[style]&quot; , &quot;'&quot; , &quot;).remove();
              nik.parents(&quot; , &quot;'&quot; , &quot;.form-group&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;);
              var radnik = $(&quot; , &quot;'&quot; , &quot;#radioNik&quot; , &quot;'&quot; , &quot;);
              radnik.attr(&quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;, false);
              radnik.attr(&quot; , &quot;'&quot; , &quot;checked&quot; , &quot;'&quot; , &quot;, false);
              radnik.prop(&quot; , &quot;'&quot; , &quot;checked&quot; , &quot;'&quot; , &quot;, false);
              radnik.parents(&quot; , &quot;'&quot; , &quot;.radio-inline&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;span[style]&quot; , &quot;'&quot; , &quot;).remove();
              radnik.parents(&quot; , &quot;'&quot; , &quot;.form-group&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;);
              setRequiredFormAlamat(false);
            }
        } else {
            $(&quot;input[name=&quot; , &quot;'&quot; , &quot;MPasien[negara_asal]&quot; , &quot;'&quot; , &quot;]&quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
            if (&quot;e-Puskesmas&quot; == &quot; , &quot;'&quot; , &quot;e-Clinic&quot; , &quot;'&quot; , &quot;) {
              var paspor = $(&quot;input[name=&quot; , &quot;'&quot; , &quot;MPasien[no_paspor]&quot; , &quot;'&quot; , &quot;]&quot;);
              paspor.attr(&quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;, false);
              paspor.parents(&quot; , &quot;'&quot; , &quot;.form-group&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;span[style]&quot; , &quot;'&quot; , &quot;).remove();
              paspor.parents(&quot; , &quot;'&quot; , &quot;.form-group&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;);
              var nik = $(&quot;input[name=&quot; , &quot;'&quot; , &quot;MPasien[nik]&quot; , &quot;'&quot; , &quot;]&quot;);
              nik.val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
              nik.attr(&quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;, false);
              nik.parents(&quot; , &quot;'&quot; , &quot;.form-group&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;span[style]&quot; , &quot;'&quot; , &quot;).remove();
              nik.parents(&quot; , &quot;'&quot; , &quot;.form-group&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;);
              var radnik = $(&quot; , &quot;'&quot; , &quot;#radioNik&quot; , &quot;'&quot; , &quot;);
              radnik.attr(&quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;, false);
              radnik.attr(&quot; , &quot;'&quot; , &quot;checked&quot; , &quot;'&quot; , &quot;, false);
              radnik.prop(&quot; , &quot;'&quot; , &quot;checked&quot; , &quot;'&quot; , &quot;, false);
              radnik.parents(&quot; , &quot;'&quot; , &quot;.radio-inline&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;span[style]&quot; , &quot;'&quot; , &quot;).remove();
              radnik.parents(&quot; , &quot;'&quot; , &quot;.form-group&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;);
              setRequiredFormAlamat(false);
            }
        }
    });

function checkradionik(obj)
{
  var radio = $(&quot; , &quot;'&quot; , &quot;#radioNik&quot; , &quot;'&quot; , &quot;);
  var nik = $(&quot;input[name=&quot; , &quot;'&quot; , &quot;MPasien[nik]&quot; , &quot;'&quot; , &quot;]&quot;);
  if ($(obj).is(&quot; , &quot;'&quot; , &quot;:checked&quot; , &quot;'&quot; , &quot;)) {
    nik.val(&quot;&quot;);
    nik.attr(&quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;, false);
    nik.parents(&quot; , &quot;'&quot; , &quot;.form-group&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;span[style]&quot; , &quot;'&quot; , &quot;).remove();
    radio.attr(&quot; , &quot;'&quot; , &quot;checked&quot; , &quot;'&quot; , &quot;, true);
  } else {
    radio.attr(&quot; , &quot;'&quot; , &quot;checked&quot; , &quot;'&quot; , &quot;, false);
    nik.val(&quot;&quot;);
    nik.attr(&quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;, true);
    nik.parents(&quot; , &quot;'&quot; , &quot;.form-group&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;span[style]&quot; , &quot;'&quot; , &quot;).remove();
    nik.parents(&quot; , &quot;'&quot; , &quot;.form-group&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;label&quot; , &quot;'&quot; , &quot;).append(&quot; , &quot;'&quot; , &quot; &lt;span style=&quot;color:red;&quot;>*&lt;/span>&quot; , &quot;'&quot; , &quot;);
  }
}

function setRequiredFormAlamat(reset = false, show = 0)
{
  var requiredInput = [&quot; , &quot;'&quot; , &quot;propinsi_id&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;kota_id&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;kecamatan_id&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;kelurahan_id&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;alamat&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;rt&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;rw&quot; , &quot;'&quot; , &quot;];
  if(reset == true){
  requiredInput.forEach(function(item,index){
    var obj = $(&quot; , &quot;'&quot; , &quot;[name=&quot;MPasien[&quot; , &quot;'&quot; , &quot;+item+&quot; , &quot;'&quot; , &quot;]&quot;]&quot; , &quot;'&quot; , &quot;);
    obj.attr(&quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;,true);
    obj.parents(&quot; , &quot;'&quot; , &quot;.form-group&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;span[style]&quot; , &quot;'&quot; , &quot;).remove();
    obj.parents(&quot; , &quot;'&quot; , &quot;.form-group&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;label&quot; , &quot;'&quot; , &quot;).append(&quot; , &quot;'&quot; , &quot; &lt;span style=&quot;color:red;&quot;>*&lt;/span>&quot; , &quot;'&quot; , &quot;);
  });    
  }else{
    requiredInput.forEach(function(item,index){
      var obj = $(&quot; , &quot;'&quot; , &quot;[name=&quot;MPasien[&quot; , &quot;'&quot; , &quot;+item+&quot; , &quot;'&quot; , &quot;]&quot;]&quot; , &quot;'&quot; , &quot;);            
      obj.removeAttr(&quot; , &quot;'&quot; , &quot;required&quot; , &quot;'&quot; , &quot;);
      obj.parents(&quot; , &quot;'&quot; , &quot;.form-group&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;span[style]&quot; , &quot;'&quot; , &quot;).remove();
    });
  }
}


    
        SUMATERA BARAT SUMATERA SELATAN SUMATERA UTARA 3 results are available, use up and down arrow keys to navigate.SUMATERA BARATSUMATERA SELATANSUMATERA UTARASUMATERA UTARAKABUPATEN KARO SUMATERA UTARA1 result is available, use up and down arrow keys to navigate.KABUPATEN KARO
        
            
                 
            
        
        
        
        
            
                
                    
                        ×
                        Memuat... 
                    
                    
                        
                        
                        
                    
                
            
        

        
            
                
                    
                        ×
                        Memuat... 
                    
                    
                        
                        
                        
                        
                    
                
            
        

        
            
                
                    
                        ×
                        Memuat... 
                    
                    
                        
                        
                        
                    
                
            
        
        
            
                
                    
                        ×
                          Notifikasi
                    
                    
                        
                        
                    
                
            
        
                                
                 Webservice BPJS sedang Gangguan    
                X
            
                                        
	#modalBantuan {
        position: fixed;
        z-index: 99998; 
        height: 52px; 
        width: 52px;
        background-color: #25d366;
        text-align: center;
        bottom: 5%;
        left: 2%;
        border-radius: 50%;
        font-size: 40px;
    }

    #modalBantuan > i {
        color: white;
        padding-top: 5px;
    }






    function openBantuan(){
        Swal.fire({
            title: &quot; , &quot;'&quot; , &quot;Apakah anda membutuhkan bantuan dari Team Customer Support?&quot; , &quot;'&quot; , &quot;,
            type: &quot; , &quot;'&quot; , &quot;info&quot; , &quot;'&quot; , &quot;,
            showCancelButton: true,
            confirmButtonColor: &quot; , &quot;'&quot; , &quot;#3085d6&quot; , &quot;'&quot; , &quot;,
            cancelButtonColor: &quot; , &quot;'&quot; , &quot;#d33&quot; , &quot;'&quot; , &quot;,
            confirmButtonText: &quot; , &quot;'&quot; , &quot;Ya&quot; , &quot;'&quot; , &quot;,
            cancelButtonText: &quot; , &quot;'&quot; , &quot;Tidak&quot; , &quot;'&quot; , &quot;
        }).then((result) => {
            if (result.value) {
                let notif_wa = &quot;https://api.whatsapp.com/send/?phone=6281277073300&amp;text=Hi,%20Saya%20pengguna%20ePuskesmas:%20,%20%20(darel)%20-%20P0000000001%20PUSKESMAS1%20-%20KABUPATEN%20BANDUNG%20memerlukan%20bantuan%20anda!&amp;app_absent=0&quot;
                window.open(notif_wa);
            }
        });
    }

    $(&quot;#modalBantuan&quot;).mousedown(function() {
        isDragging = false;
    }).mousemove(function() {
        isDragging = true;
    }).mouseup(function() {
        var wasDragging = isDragging;
        isDragging = false;
        if (!wasDragging) {
            openBantuan();
        }
    });

    dragElement(document.getElementById(&quot;modalBantuan&quot;));

    function dragElement(elmnt) {
      var pos1 = 0, pos2 = 0, pos3 = 0, pos4 = 0;
      if (document.getElementById(elmnt.id + &quot;header&quot;)) {
        document.getElementById(elmnt.id + &quot;header&quot;).onmousedown = dragMouseDown;
      } else {
        elmnt.onmousedown = dragMouseDown;
      }

      function dragMouseDown(e) {
        e = e || window.event;
        e.preventDefault();
        pos3 = e.clientX;
        pos4 = e.clientY;
        document.onmouseup = closeDragElement;
        document.onmousemove = elementDrag;
      }

      function elementDrag(e) {
        e = e || window.event;
        e.preventDefault();
        pos1 = pos3 - e.clientX;
        pos2 = pos4 - e.clientY;
        pos3 = e.clientX;
        pos4 = e.clientY;
        elmnt.style.top = (elmnt.offsetTop - pos2) + &quot;px&quot;;
        elmnt.style.left = (elmnt.offsetLeft - pos1) + &quot;px&quot;;
      }

      function closeDragElement() {
        document.onmouseup = null;
        document.onmousemove = null;
      }
    }

    var modalBantuanDrag = document.getElementById(&quot; , &quot;'&quot; , &quot;modalBantuan&quot; , &quot;'&quot; , &quot;);
    modalBantuanDrag.addEventListener(&quot; , &quot;'&quot; , &quot;touchmove&quot; , &quot;'&quot; , &quot;, function(event) {
      event.preventDefault();
      if (event.targetTouches.length == 1) {
        var touch = event.targetTouches[0];
        modalBantuanDrag.style.left = touch.pageX + &quot; , &quot;'&quot; , &quot;px&quot; , &quot;'&quot; , &quot;;
        modalBantuanDrag.style.top = touch.pageY + &quot; , &quot;'&quot; , &quot;px&quot; , &quot;'&quot; , &quot;;
      }
    }, {passive: true});
                
            $(document).ready(function(){
                $(window).scroll(function () {
                    $(&quot; , &quot;'&quot; , &quot;#icon_back&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;style&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;margin-top:&quot; , &quot;'&quot; , &quot;+(parseInt(innerHeight)-150)+&quot; , &quot;'&quot; , &quot;px;margin-left:&quot; , &quot;'&quot; , &quot;+(parseInt(innerWidth)-70)+&quot; , &quot;'&quot; , &quot;px;z-index:1000;&quot; , &quot;'&quot; , &quot;);
                    if ($(this).scrollTop() > 125) {
                        $(&quot; , &quot;'&quot; , &quot;#top-link-block&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;);
                        $(&quot; , &quot;'&quot; , &quot;#top-link-block&quot; , &quot;'&quot; , &quot;).fadeIn();
                    }else{
                        $(&quot; , &quot;'&quot; , &quot;#top-link-block&quot; , &quot;'&quot; , &quot;).fadeOut();
                    }                    
                });

                let message_skrining= &quot; , &quot;'&quot; , &quot;null&quot; , &quot;'&quot; , &quot;;
                if(message_skrining == &quot;error_message&quot;){

                    let text_message = &quot; , &quot;'&quot; , &quot;null&quot; , &quot;'&quot; , &quot;;
                    let text_status = &quot; , &quot;'&quot; , &quot;null&quot; , &quot;'&quot; , &quot;;
                    if(text_status == &quot; , &quot;'&quot; , &quot;null&quot; , &quot;'&quot; , &quot;){
                        text_status = &quot; , &quot;'&quot; , &quot;error&quot; , &quot;'&quot; , &quot;;
                    }
                    if(text_message != &quot; , &quot;'&quot; , &quot;null&quot; , &quot;'&quot; , &quot;){
                        Swal.fire(
                          text_message,
                          &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;,
                          text_status
                        )
                    }
                }

                $(&quot; , &quot;'&quot; , &quot;[data-toggle=&quot;tooltip&quot;]&quot; , &quot;'&quot; , &quot;).tooltip();


                                                                                        let stt= localStorage.getItem(&quot;count_notif&quot;);
                            if(stt !== &quot;undefined&quot; &amp;&amp; stt !== &quot;0&quot; &amp;&amp; stt !== &quot;null&quot; ){
                                $(&quot;#kolom_gagal_bridging&quot;).html(stt);   
                            }
                                                                            
            });
            
            function aksipesan(){
                Swal.fire({
                    text: &quot;Apakah Anda ingin tutup pesan Webservice bpjs dan tidak di tampilkan lagi ?&quot;,
                    type: &quot; , &quot;'&quot; , &quot;warning&quot; , &quot;'&quot; , &quot;,
                    showCancelButton: true,
                    confirmButtonColor: &quot; , &quot;'&quot; , &quot;#3085d6&quot; , &quot;'&quot; , &quot;,
                    cancelButtonColor: &quot; , &quot;'&quot; , &quot;#d33&quot; , &quot;'&quot; , &quot;,
                    confirmButtonText: &quot; , &quot;'&quot; , &quot;Ya&quot; , &quot;'&quot; , &quot;,
                    cancelButtonText: &quot; , &quot;'&quot; , &quot;Tutup Sekarang Saja&quot; , &quot;'&quot; , &quot;
                }).then((result) => {
                    if(result.value){
                        setSessionWebSocket(1, &quot;indikatorbpjs&quot;);
                        $(&quot; , &quot;'&quot; , &quot;#notif-connection-bpjs&quot; , &quot;'&quot; , &quot;).hide();
                    }

                    if (result.dismiss == &quot;cancel&quot;) {
                        $(&quot; , &quot;'&quot; , &quot;#notif-connection-bpjs&quot; , &quot;'&quot; , &quot;).hide();
                    }
                });
            }

                        function callAntreanKiosk(nomorAntrean, ruangan_code){
                                return false;
            }
            
            function deletePasienKiosk(pasien_id){
                                return false;
            }

            function deletePendaftaranKiosk(pendaftaran_id){
                                return false;
            }

            function syncPasienKiosk(){
                                return false;
            }

            function syncPendaftaranKiosk(){
                                return false;
            }
        
        
            var date = new Date();
            var tanggal = new Date(date.getFullYear(), date.getMonth(), date.getDate(), 23, 59, 59, 59);
            var bulan = new Date(date.getFullYear(), date.getMonth(), 1, 0, 0, 0);
            var tahun = new Date(date.getFullYear(), 1, 0, 0, 0);
                            var tanggal_min = new Date(date.getFullYear(), date.getMonth(), date.getDate(), 00, 00, 00, 00);
                        var sistole_diastole_anamnesa = &quot;&quot;;
            var default_sistole = &quot;&quot;;
            var default_diastole = &quot;&quot;;
        
        
                    
            
                
            function showUpdateLog(from_menu = false)
            {
                $.ajax({
                    url: &quot;https://demo.epuskesmas.id/updatelog/showupdatelog&quot;,
                    method: &quot; , &quot;'&quot; , &quot;GET&quot; , &quot;'&quot; , &quot;,
                    dataType: &quot;json&quot;,
                    data: {
                        _token: &quot;1Qzy4frvPTBhCAOEI2lL6VeUfUPoVgVDUjUncrzY&quot;
                    },
                    success: function(data) {
                        if(data.title != &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot; &amp;&amp; data.title != undefined){
                            $(&quot; , &quot;'&quot; , &quot;#modal .modal-title&quot; , &quot;'&quot; , &quot;).html(data.title);
                            $(&quot; , &quot;'&quot; , &quot;#modal .modal-form&quot; , &quot;'&quot; , &quot;).html(data.form);
                            openmodal(&quot;modal-lg&quot;);
                        }else{
                            if(from_menu){
                                alert(&quot;Belum ada update terbaru saat ini!&quot;,&quot;info&quot;);
                            }
                        }
                    },
                    error: function (xhr, ajaxOptions, thrownError) {
                        alert(&quot;Terjadi kesalahan sistem, silahkan hubungi team support kami!&quot;, &quot;danger&quot;);
                        console.log(xhr.status);
                        console.log(thrownError);
                    }
                });
            }

            function showNotif(from_menu = false)
            {
                $.ajax({
                    url: &quot;https://demo.epuskesmas.id/mpuskesmas/shownotif&quot;,
                    method: &quot; , &quot;'&quot; , &quot;GET&quot; , &quot;'&quot; , &quot;,
                    dataType: &quot;json&quot;,
                    data: {
                        _token: &quot;1Qzy4frvPTBhCAOEI2lL6VeUfUPoVgVDUjUncrzY&quot;
                    },
                    success: function(data) {
                        if(data.title != &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot; &amp;&amp; data.title != undefined){
                            $(&quot; , &quot;'&quot; , &quot;#modal_notif .modal-title&quot; , &quot;'&quot; , &quot;).html(data.title);
                            $(&quot; , &quot;'&quot; , &quot;#modal_notif .modal-form&quot; , &quot;'&quot; , &quot;).html(data.form);
                            openmodal_notif(&quot;modal-lg&quot;);

                            if (data.notif==&quot; , &quot;'&quot; , &quot;NOTIF_02&quot; , &quot;'&quot; , &quot;) {
                                $(&quot; , &quot;'&quot; , &quot;#btn_close_suspend&quot; , &quot;'&quot; , &quot;).hide();
                            }

                        }else{
                            if(from_menu){
                                alert(&quot;Belum ada update terbaru saat ini!&quot;,&quot;info&quot;);
                            }
                        }
                    },
                    error: function (xhr, ajaxOptions, thrownError) {
                        alert(&quot;Terjadi kesalahan sistem, silahkan hubungi team support kami!&quot;, &quot;danger&quot;);
                        console.log(xhr.status);
                        console.log(thrownError);
                    }
                });
            }

            function showNotifBridging(from_menu = false)
            {
                $.blockUI({
                css: {
                    border: &quot; , &quot;'&quot; , &quot;none&quot; , &quot;'&quot; , &quot;,
                    padding: &quot; , &quot;'&quot; , &quot;15px&quot; , &quot;'&quot; , &quot;,
                    backgroundColor: &quot; , &quot;'&quot; , &quot;#000&quot; , &quot;'&quot; , &quot;,
                    &quot; , &quot;'&quot; , &quot;-webkit-border-radius&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;10px&quot; , &quot;'&quot; , &quot;,
                    &quot; , &quot;'&quot; , &quot;-moz-border-radius&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;10px&quot; , &quot;'&quot; , &quot;,
                    opacity: .5,
                    fontSize: &quot; , &quot;'&quot; , &quot;8px&quot; , &quot;'&quot; , &quot;,
                    color: &quot; , &quot;'&quot; , &quot;#fff&quot; , &quot;'&quot; , &quot;
                    },
                    message: &quot; , &quot;'&quot; , &quot;&lt;h3>Memuat....&lt;/h3>&quot; , &quot;'&quot; , &quot;
                });

                $.ajax({
                    url: &quot;https://demo.epuskesmas.id/home/shownotifbridging&quot;,
                    method: &quot; , &quot;'&quot; , &quot;GET&quot; , &quot;'&quot; , &quot;,
                    dataType: &quot;json&quot;,
                    data: {
                        _token: &quot;1Qzy4frvPTBhCAOEI2lL6VeUfUPoVgVDUjUncrzY&quot;
                    },
                    success: function(data) {
                        $(&quot; , &quot;'&quot; , &quot;#modal_notif_bridging .modal-title&quot; , &quot;'&quot; , &quot;).html(data.title);
                        $(&quot; , &quot;'&quot; , &quot;#modal_notif_bridging .modal-form&quot; , &quot;'&quot; , &quot;).html(data.form);
                        openmodal_notif_bridging(&quot;modal-lg&quot;);
                    },
                    error: function (xhr, ajaxOptions, thrownError) {
                        alert(&quot;Terjadi kesalahan sistem, silahkan hubungi team support kami!&quot;, &quot;danger&quot;);
                        console.log(xhr.status);
                        console.log(thrownError);
                    }
                });
            }

            function gagalBridgingCount(from_menu = false)
            {
                $.ajax({
                    url: &quot;https://demo.epuskesmas.id/home/gagalbridgingcount&quot;,
                    method: &quot; , &quot;'&quot; , &quot;GET&quot; , &quot;'&quot; , &quot;,
                    dataType: &quot;json&quot;,
                    data: {
                        _token: &quot;1Qzy4frvPTBhCAOEI2lL6VeUfUPoVgVDUjUncrzY&quot;
                    },
                    success: function(data) {
                        var x = document.getElementById(&quot;button_gagal_bridging&quot;);
                        if(data > 0){
                            $(&quot;#kolom_gagal_bridging&quot;).html(data);
                            localStorage.setItem(&quot; , &quot;'&quot; , &quot;count_notif&quot; , &quot;'&quot; , &quot;,data);    
                        }else{
                            localStorage.setItem(&quot; , &quot;'&quot; , &quot;count_notif&quot; , &quot;'&quot; , &quot;,&quot;0&quot;);
                        }
                    },
                    error: function (xhr, ajaxOptions, thrownError) {
                        alert(&quot;Terjadi kesalahan sistem, silahkan hubungi team support kami!&quot;, &quot;danger&quot;);
                        console.log(xhr.status);
                        console.log(thrownError);
                    }
                });
            }

        function showRiwayatPelayanan(obj)
        {
            data = $(obj).data();
            $.ajax({
                url: &quot;https://demo.epuskesmas.id/pelayanan/showriwayatpelayanan&quot;,
                method: &quot;GET&quot;,
                dataType: &quot;json&quot;,
                data: {
                id: data.id,
                puskesmas: data.puskesmas,
                _token: &quot;1Qzy4frvPTBhCAOEI2lL6VeUfUPoVgVDUjUncrzY&quot;,
            },
            success: function(data) {
                $(&quot;#modal .modal-title&quot;).html(data.title);
                $(&quot;#modal .modal-form&quot;).html(data.form);
                openmodal(&quot;modal-lg&quot;);
            },
            error: function (xhr, ajaxOptions, thrownError) {
                    alert(&quot;Terjadi kesalahan sistem, silahkan hubungi team support kami!&quot;, &quot;danger&quot;);
                    console.log(xhr.status);
                    console.log(thrownError);
                }
            });
        }

        function showRiwayatKunjunganBpjs(obj)
        {
            data = $(obj).data();
            $.ajax({
                url: &quot;https://demo.epuskesmas.id/pelayanan/showriwayatkunjunganbpjs&quot;,
                method: &quot;GET&quot;,
                dataType: &quot;json&quot;,
                data: {
                    id: data.id,
                    _token: &quot;1Qzy4frvPTBhCAOEI2lL6VeUfUPoVgVDUjUncrzY&quot;,
                },
                success: function(data) {
                    $(&quot;#modal .modal-title&quot;).html(data.title);
                    $(&quot;#modal .modal-form&quot;).html(data.form);
                    openmodal(&quot;modal-lg&quot;);
                },
                error: function (xhr, ajaxOptions, thrownError) {
                        alert(&quot;Terjadi kesalahan sistem, silahkan hubungi team support kami!&quot;, &quot;danger&quot;);
                        console.log(xhr.status);
                        console.log(thrownError);
                }
            });
        }

        function getRiwayatKunjunganBpjs(obj)
        {
            data = $(obj).data();
            $.ajax({
                url: &quot;https://demo.epuskesmas.id/pelayanan/getriwayatkunjunganbpjs&quot;,
                method: &quot;GET&quot;,
                dataType: &quot;json&quot;,
                data: {
                    noasuransi: data.noasuransi,
                    _token: &quot;1Qzy4frvPTBhCAOEI2lL6VeUfUPoVgVDUjUncrzY&quot;,
                },
                success: function(data) {
                    alert(data.message, data.status);
                    $(obj).removeClass(&quot; , &quot;'&quot; , &quot;loading&quot; , &quot;'&quot; , &quot;);
                    if(data.status == &quot;success&quot;){
                        window.location.reload(true);
                    }
                },
                error: function (xhr, ajaxOptions, thrownError) {
                    alert(&quot;Terjadi kesalahan sistem, silahkan hubungi team support kami!&quot;, &quot;danger&quot;);
                    console.log(xhr.status);
                    console.log(thrownError);
                }
            });
        }

        function setSessionWebSocket(action,to = null)
        {
            $.ajax({
                url: &quot;https://demo.epuskesmas.id/home/setsessionwebsocket&quot;,
                method: &quot; , &quot;'&quot; , &quot;POST&quot; , &quot;'&quot; , &quot;,
                dataType: &quot;json&quot;,
                data: {
                    action: action,
                    for_customsession:to,
                    _token: &quot;1Qzy4frvPTBhCAOEI2lL6VeUfUPoVgVDUjUncrzY&quot;
                },
                success: function(data) {
                    console.log(&quot;success&quot;);
                },
                error: function (xhr, ajaxOptions, thrownError) {
                    alert(&quot;Terjadi kesalahan sistem, silahkan hubungi team support kami!&quot;, &quot;danger&quot;);
                }
            });
        }

        
        

        
                        
                    $(`#checkEnvConfig`).html(&quot; , &quot;'&quot; , &quot;&lt;center>Dashboard antrian realtime belum di setting&lt;/center>&quot; , &quot;'&quot; , &quot;);
                    $(`#checkEnvConfigRanap`).html(&quot; , &quot;'&quot; , &quot;&lt;center>Dashboard rawat inap realtime belum di setting&lt;/center>&quot; , &quot;'&quot; , &quot;);
                
                        

id(&quot;form_creates&quot;)/div[@class=&quot;panel-body row&quot;]/div[@class=&quot;col-sm-4&quot;]/div[@class=&quot;panel panel-default container-fluid&quot;]/div[@class=&quot;panel-body row&quot;]/div[@class=&quot;form-group&quot;]/div[@class=&quot;col-sm-8 has-error&quot;]/input[@class=&quot;form-control input-sm uppercase ui-autocomplete-input&quot;]4 / 64&quot;))]</value>
      <webElementGuid>d0fff7d9-1bad-4143-ac1c-8a11c5597a7a</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
