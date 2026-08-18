const layers=[
 {year:"2026",code:"LAYER 01",title:"THE OPTIMIZED PRESENT",depth:"0.0",className:"layer-2026",note:"The newest layer is usually the loudest. It is rarely the deepest.",artifacts:[["01","GLASS BUTTON","Polished until the label disappeared"],["02","ENGAGEMENT PILL","Designed to contain one metric"],["03","AI SUMMARY","Confidently compressed from six tabs"]],html:`<div class="site26-nav"><b>PLAIN/FORM</b><div><span>Product</span><span>Stories</span><span>About</span></div><button>Join waitlist</button></div><div class="site26-hero"><span class="site26-pill">A NEW WAY TO EXPERIENCE EVERYTHING</span><h3>Less interface.<br>More <i>possibility.</i></h3><p>The intelligent workspace for work that works around you.</p><button>Start becoming →</button></div><div class="site26-cards"><i></i><i></i><i></i></div>`},
 {year:"2012",code:"LAYER 02",title:"THE RESPONSIVE TURN",depth:"14.8",className:"layer-2012",note:"This stratum believed every problem could be solved with a grid, a gradient, and three equal columns.",artifacts:[["04","HAMBURGER ICON","Navigation folded into three lines"],["05","HERO CAROUSEL","Four messages, none remembered"],["06","SOCIAL COUNTER","Proof that 2,481 people had clicked"]],html:`<div class="site12-bar"><b>BRIGHTLY</b><span>Home &nbsp; Features &nbsp; Pricing &nbsp; Blog</span></div><div class="site12-hero"><div class="site12-copy"><h3>BIG IDEAS.<br>ANY SCREEN.</h3><p>A clean, responsive template for startups, apps, agencies and people with very large call-to-action buttons.</p><button>LEARN MORE</button></div><div class="site12-graphic">◫</div></div><div class="site12-features"><span><b>RESPONSIVE</b><br>Looks good everywhere.</span><span><b>RETINA READY</b><br>Twice as many pixels.</span><span><b>SOCIAL</b><br>Share this immediately.</span></div>`},
 {year:"2003",code:"LAYER 03",title:"THE PERSONAL WEB",depth:"31.6",className:"layer-2003",note:"Before platforms became places, a homepage was a room someone decorated and left unlocked.",artifacts:[["07","GUESTBOOK ENTRY","hi cool site sign mine pls"],["08","UNDER CONSTRUCTION","A promise rendered as a tiny worker"],["09","MIDI AUTOPLAY","The sound of arriving unannounced"]],html:`<div class="site03-window"><div class="site03-title">✦ jean's corner of the internet ✦ — Microsoft Internet Explorer</div><div class="site03-body"><nav class="site03-nav"><b>~* navigation *~</b><a>home</a><a>about me!!</a><a>my blinkies</a><a>cool links</a><a>guestbook</a><a>email me</a></nav><div class="site03-content"><marquee>★ WELCOME TO MY HOMEPAGE ★ BEST VIEWED WITH YOUR EYES OPEN ★</marquee><h3>Hello, internet!</h3><hr><p>You are visitor <b>000042</b>. This is my website about things I like, things I made, and probably cats.</p><div class="site03-gif">UNDER<br>CONSTRUCTION</div><div class="site03-gif">100%<br>HAND CODED</div><p><a href="#">sign my guestbook!!!</a></p></div></div></div>`},
 {year:"1996",code:"LAYER 04",title:"THE HANDMADE SOURCE",depth:"52.4",className:"layer-1996",note:"At bedrock, design and code are the same gesture. View Source is both instruction and invitation.",artifacts:[["10","TABLE LAYOUT","The first dependable architecture"],["11","WEBRING SHARD","A door to somebody else's page"],["12","VIEW SOURCE","The oldest surviving tutorial"]],html:`<div class="site96-table"><div class="site96-banner">TAYLOR'S WORLD WIDE WEB HOME PAGE</div><div class="site96-main"><nav class="site96-links"><b>CONTENTS</b><a>About Me</a><a>My Projects</a><a>Cool Sites</a><a>Web Ring</a><a>E-Mail</a></nav><div class="site96-copy"><div class="site96-stars">★ ★ ★ ★ ★</div><h3>Welcome, net traveler!</h3><p>This page was written by hand in Notepad. There are no cookies, no feed, and no algorithm deciding what you see next.</p><p>I hope you find something interesting here.</p><div class="site96-counter">YOU ARE VISITOR 0000009</div><br><a style="color:#00ffff">[ VIEW SOURCE ]</a></div></div><marquee>THIS SITE IS ALWAYS UNDER CONSTRUCTION — COME BACK SOON!</marquee></div>`}
];

const state={layer:0,revealed:0,brush:64,drawing:false,finds:[],points:[],complete:false};
const $=s=>document.querySelector(s),canvas=$("#surface"),ctx=canvas.getContext("2d",{willReadFrequently:true}),field=$("#field"),brush=$("#brush"),continueBtn=$("#continue");

function buildStrata(){
 const list=$("#strata-list");
 list.innerHTML=layers.map((l,i)=>`<li class="${i===state.layer?'active':i<state.layer?'complete':''}"><span>${l.year}</span><b>${l.title}</b></li>`).join("");
}

function drawSurface(){
 const ratio=Math.min(window.devicePixelRatio||1,2),r=field.getBoundingClientRect();
 canvas.width=Math.floor(r.width*ratio);canvas.height=Math.floor(r.height*ratio);ctx.setTransform(ratio,0,0,ratio,0,0);
 const g=ctx.createLinearGradient(0,0,r.width,r.height);g.addColorStop(0,"#1b1d19");g.addColorStop(.5,"#2a2a24");g.addColorStop(1,"#12140f");ctx.globalCompositeOperation="source-over";ctx.fillStyle=g;ctx.fillRect(0,0,r.width,r.height);
 ctx.fillStyle="rgba(236,233,223,.06)";for(let i=0;i<900;i++){const x=Math.random()*r.width,y=Math.random()*r.height,s=Math.random()*1.8;ctx.fillRect(x,y,s,s)}
 ctx.strokeStyle="rgba(215,255,63,.12)";ctx.lineWidth=1;for(let x=0;x<r.width;x+=54){ctx.beginPath();ctx.moveTo(x,0);ctx.lineTo(x,r.height);ctx.stroke()}for(let y=0;y<r.height;y+=54){ctx.beginPath();ctx.moveTo(0,y);ctx.lineTo(r.width,y);ctx.stroke()}
 ctx.fillStyle="#ece9df";ctx.font="500 10px DM Mono";ctx.fillText(`SURFACE ${layers[state.layer].year} / DRAG TO REMOVE`,24,32);ctx.font="400 7px DM Mono";ctx.fillStyle="rgba(236,233,223,.52)";ctx.fillText("INTERFACE SEDIMENT / COMPOSITE MATERIAL",24,49);
 state.revealed=0;state.points=[];state.complete=false;updateProgress();
}

function loadLayer(){
 const layer=layers[state.layer],site=$("#buried-site");site.className=`buried-site ${layer.className}`;site.innerHTML=layer.html;
 $("#layer-code").textContent=layer.code;$("#layer-title").textContent=layer.title;$("#depth").innerHTML=`${layer.depth}<span>cm</span>`;$("#field-note").textContent=layer.note;continueBtn.classList.remove("ready");continueBtn.innerHTML=state.layer===layers.length-1?"CONSERVE THE SITE <i>✦</i>":"DESCEND TO NEXT STRATUM <i>↓</i>";buildStrata();requestAnimationFrame(drawSurface);
}

function localPoint(e){const r=canvas.getBoundingClientRect();return{x:e.clientX-r.left,y:e.clientY-r.top}}
function excavate(x,y,size=state.brush){
 ctx.save();ctx.globalCompositeOperation="destination-out";const g=ctx.createRadialGradient(x,y,size*.12,x,y,size*.5);g.addColorStop(0,"rgba(0,0,0,1)");g.addColorStop(.72,"rgba(0,0,0,.92)");g.addColorStop(1,"rgba(0,0,0,0)");ctx.fillStyle=g;ctx.beginPath();ctx.arc(x,y,size*.52,0,Math.PI*2);ctx.fill();ctx.restore();
 state.points.push([x,y]);if(state.points.length%7===0)measure();
}
function measure(){
 const r=field.getBoundingClientRect(),sample=ctx.getImageData(0,0,canvas.width,canvas.height).data;let transparent=0,total=0;for(let i=3;i<sample.length;i+=64){total++;if(sample[i]<80)transparent++}state.revealed=Math.min(100,Math.round(transparent/total*100));updateProgress();
 const artifactIndex=state.revealed>=61?2:state.revealed>=35?1:state.revealed>=14?0:-1;if(artifactIndex>=0)recover(state.layer*3+artifactIndex);
 if(state.revealed>=68&&!state.complete){state.complete=true;continueBtn.classList.add("ready");}
}
function updateProgress(){$("#percent").textContent=String(state.revealed).padStart(2,"0")+"%"}
function recover(globalIndex){
 if(state.finds.includes(globalIndex))return;state.finds.push(globalIndex);const item=layers[Math.floor(globalIndex/3)].artifacts[globalIndex%3],card=document.createElement("div");card.className="find-card";card.innerHTML=`<i>${item[0]}</i><b>${item[1]}</b><span>${item[2]}</span>`;const list=$("#finds-list");if(list.querySelector("p"))list.innerHTML="";list.prepend(card);$("#find-count").textContent=state.finds.length;
}
function sweep(){
 const r=field.getBoundingClientRect(),y=r.height*(.2+Math.random()*.6);let x=0;const timer=setInterval(()=>{for(let i=0;i<3;i++)excavate(x+i*22,y+Math.sin((x+i*22)/42)*35,76);x+=28;if(x>r.width){clearInterval(timer);measure()}},16);
}
function descend(){
 if(state.layer<layers.length-1){field.animate([{opacity:1,filter:"blur(0)"},{opacity:0,filter:"blur(12px)"}],{duration:420}).finished.then(()=>{state.layer++;loadLayer();field.animate([{opacity:0},{opacity:1}],{duration:500})})}else{layers[state.layer].artifacts.forEach((_,i)=>recover(state.layer*3+i));const d=$("#archive-dialog");$("#archive-summary").innerHTML=layers.map(l=>`<span>${l.year} / RECOVERED</span>`).join("");d.showModal()}
}

canvas.addEventListener("pointerdown",e=>{state.drawing=true;canvas.setPointerCapture(e.pointerId);const p=localPoint(e);excavate(p.x,p.y)});canvas.addEventListener("pointermove",e=>{const p=localPoint(e);brush.style.left=p.x+"px";brush.style.top=p.y+"px";if(state.drawing)excavate(p.x,p.y)});canvas.addEventListener("pointerup",()=>{state.drawing=false;measure()});canvas.addEventListener("pointercancel",()=>state.drawing=false);
$("#brush-size").addEventListener("input",e=>{state.brush=+e.target.value;$("#brush-value").textContent=state.brush;brush.style.width=brush.style.height=state.brush+"px"});$("#sweep").onclick=sweep;$("#reset").onclick=drawSurface;continueBtn.onclick=descend;window.addEventListener("resize",()=>{clearTimeout(window.__waResize);window.__waResize=setTimeout(drawSurface,180)});$(".dialog-close").onclick=()=>$("#archive-dialog").close();$(".restart-all").onclick=()=>{state.layer=0;state.finds=[];$("#finds-list").innerHTML="<p>No fragments recovered.<br>Begin at the surface.</p>";$("#find-count").textContent="0";$("#archive-dialog").close();loadLayer()};
loadLayer();
