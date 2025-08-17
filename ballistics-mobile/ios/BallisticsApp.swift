import SwiftUI
import UIKit

@main
struct BallisticsApp: App {
    var body: some Scene {
        WindowGroup {
            BallisticsView()
        }
    }
}

struct BallisticsView: UIViewControllerRepresentable {
    func makeUIViewController(context: Context) -> BallisticsViewController {
        return BallisticsViewController()
    }
    
    func updateUIViewController(_ uiViewController: BallisticsViewController, context: Context) {
    }
}

class BallisticsViewController: UIViewController {
    private var appPtr: UnsafeMutableRawPointer?
    private var displayLink: CADisplayLink?
    
    override func viewDidLoad() {
        super.viewDidLoad()
        
        // Initialize Rust app
        let documentsPath = NSSearchPathForDirectoriesInDomains(
            .documentDirectory,
            .userDomainMask,
            true
        ).first!
        
        appPtr = ballistics_init(documentsPath)
        
        // Setup display link for rendering
        displayLink = CADisplayLink(target: self, selector: #selector(render))
        displayLink?.add(to: .current, forMode: .default)
        
        // Setup gesture recognizers
        let panGesture = UIPanGestureRecognizer(target: self, action: #selector(handlePan))
        view.addGestureRecognizer(panGesture)
        
        let tapGesture = UITapGestureRecognizer(target: self, action: #selector(handleTap))
        view.addGestureRecognizer(tapGesture)
    }
    
    @objc private func render() {
        guard let ptr = appPtr else { return }
        ballistics_update(ptr)
        ballistics_render(ptr)
    }
    
    @objc private func handlePan(_ gesture: UIPanGestureRecognizer) {
        guard let ptr = appPtr else { return }
        let location = gesture.location(in: view)
        
        let eventType: Int32
        switch gesture.state {
        case .began:
            eventType = 0
        case .changed:
            eventType = 1
        case .ended:
            eventType = 2
        case .cancelled:
            eventType = 3
        default:
            return
        }
        
        ballistics_touch_event(ptr, Float(location.x), Float(location.y), eventType)
    }
    
    @objc private func handleTap(_ gesture: UITapGestureRecognizer) {
        guard let ptr = appPtr else { return }
        let location = gesture.location(in: view)
        
        ballistics_touch_event(ptr, Float(location.x), Float(location.y), 0)
        ballistics_touch_event(ptr, Float(location.x), Float(location.y), 2)
    }
    
    override func viewWillAppear(_ animated: Bool) {
        super.viewWillAppear(animated)
        if let ptr = appPtr {
            ballistics_on_resume(ptr)
        }
    }
    
    override func viewWillDisappear(_ animated: Bool) {
        super.viewWillDisappear(animated)
        if let ptr = appPtr {
            ballistics_on_pause(ptr)
        }
    }
    
    deinit {
        displayLink?.invalidate()
        if let ptr = appPtr {
            ballistics_destroy(ptr)
        }
    }
}

// Location Service
@objc class LocationService: NSObject {
    static let shared = LocationService()
    
    @objc func startLocationUpdates() {
        // Implement CoreLocation
    }
    
    @objc func stopLocationUpdates() {
        // Stop location updates
    }
}

// Camera Service
@objc class CameraService: NSObject {
    static let shared = CameraService()
    
    @objc func takePhoto() {
        // Implement UIImagePickerController
    }
}

// Share Service
@objc class ShareService: NSObject {
    static let shared = ShareService()
    
    @objc func shareText(_ text: String, withTitle title: String) {
        let activityVC = UIActivityViewController(
            activityItems: [text],
            applicationActivities: nil
        )
        
        if let windowScene = UIApplication.shared.connectedScenes.first as? UIWindowScene,
           let rootVC = windowScene.windows.first?.rootViewController {
            rootVC.present(activityVC, animated: true)
        }
    }
}