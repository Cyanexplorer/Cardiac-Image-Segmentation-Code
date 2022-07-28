const fs = require("fs");
const path = require("path");
const util = require('util');
const exec = util.promisify(require('child_process').execFile);

class dicom2raw {
	constructor(dcmFolder, tmpFolder, outputModelPath){
		this.ijPath = ijPath
		this.marco = ''
		this.finish = false
		
		this.dcmFolder = dcmFolder
		this.tmpFolder = tmpFolder
		this.outputModelPath = outputModelPath
		
		this.timestamp = new Date().getTime()
	}
	
	mkScript(){
		this.marco = `dcmDir = '${this.dcmFolder}/';\ 
						fileList = getFileList(dcmDir);\
						path = dcmDir + fileList[0];\
						run('Bio-Formats Importer', 'open=[path] autoscale color_mode=Default rois_import=[ROI manager] view=Hyperstack stack_order=XYCZT');\ 
						convertTo8Bit();\
						infoString = getMetadata('Info');\
						metaTxt = File.open('${this.tmpFolder}/${this.timestamp}.txt'); \
						print(metaTxt, infoString);\
						saveAs('stl', '${this.tmpFolder}/${this.timestamp}.raw'); \
						run('Quit');`
		return true
	}
	
	async convert(){
		try{
			await exec('/convert.py', ['-eval', this.marco])
		}
		catch(e){
			console.log(path.resolve(this.ijPath + '/convert.py'))
		}
		
		const isRawFile = fs.existsSync(`${this.tmpFolder}/${this.timestamp}.stl`)
		const isMetaFile = fs.existsSync(`${this.tmpFolder}/${this.timestamp}.txt`)
		
		if(isRawFile && isMetaFile)
			this.finish = true
		
		return this.finish
	}
	
	getMetaSize(){
		if(!this.finish)
			return false
		
		const metaString = fs.readFileSync(`${this.tmpFolder}/${this.timestamp}.txt`, 'utf-8')
		
		fs.unlinkSync(`${this.tmpFolder}/${this.timestamp}.txt`)
		
		return metaString
				.split('\n')
				.filter(e => /Size(X|Y|Z)/.test(e))
				.reduce((a,e) => {
					const [k,v] = e.split('=')
					a[k.replace(/\s+|size/ig, '')]=v.replace(/\s+/g, '')

					return a;
				}, {})
	}
	
	getModelOutput(outputModelPath){
		if(!this.finish)
			return false
		
		fs.renameSync(
			`${this.tmpFolder}/${this.timestamp}.stl`,
			typeof outputModelPath == 'undefined' ? this.outputModelPath : outputModelPath
		)
		
		return true
	}


}

module.exports = dicom2raw