(async () => {
  console.log('Stimulate heavy work')
  await new Promise(resolve => setTimeout(resolve, 1000 * Math.random()))
  console.log('Stimulate heavy work done')
})()
